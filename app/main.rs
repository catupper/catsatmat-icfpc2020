use app::*;
use http_body::Body as _;
use log::{error, info};

use hyper::{Body, Client, Method, Request, StatusCode};

use std::env;
use std::process;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

async fn sample(server_url: &str, player_key: &str) -> Result<()> {
    let client = Client::new();
    let req = Request::builder()
        .method(Method::POST)
        .uri(server_url)
        .body(Body::from(player_key.to_string()))?;

    info!("ServerUrl: {}; PlayerKey: {}", server_url, player_key);

    match client.request(req).await {
        Ok(mut res) => match res.status() {
            StatusCode::OK => {
                info!("Server response: ");
                while let Some(chunk) = res.body_mut().data().await {
                    match chunk {
                        Ok(content) => info!("{:?}", content),
                        Err(why) => error!("error reading body: {:?}", why),
                    }
                }
            }
            _ => {
                info!("Unexpected server response:");
                info!("HTTP code: {}", res.status());
                info!("Response body: ");
                while let Some(chunk) = res.body_mut().data().await {
                    match chunk {
                        Ok(content) => info!("{:?}", content),
                        Err(why) => error!("error reading body: {:?}", why),
                    }
                }
                process::exit(2);
            }
        },
        Err(err) => {
            error!("Unexpected server response:\n{}", err);
            process::exit(1);
        }
    }

    Ok(())
}

const DEFAULT_URL: &str = "https://icfpc2020-api.testkontur.ru";
const API_KEY: &str = "41ff8e29e5fa4596928186fcfe5bfee2";
const DEFAULT_PLAYER_KEY: &str = "1";

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::Builder::from_default_env()
        .format_timestamp_millis()
        .init();

    let args: Vec<String> = env::args().collect();

    let server_url = args
        .get(1)
        .cloned()
        .unwrap_or_else(|| DEFAULT_URL.to_string());
    let player_key = args
        .get(2)
        .cloned()
        .unwrap_or_else(|| DEFAULT_PLAYER_KEY.to_string());
    if player_key != DEFAULT_PLAYER_KEY {
        sample(&server_url, &player_key).await?;
    }
    let sender = Sender::new(server_url.to_string(), API_KEY.to_string());
    let player_key = player_key.parse()?;
    let response = sender.join(player_key).await?;
    let (current_game_stage, list_a, _state) = response.as_game_response();
    let mut game_stage = current_game_stage;
    let is_defender = list_a.cdr().car();
    info!("IsDefneder: {}", is_defender);
    assert_eq!(game_stage, 0);
    if is_defender == Expr::Int(1) {
        let response = sender.start(player_key, 1, 1, 2, 2).await?;
        let (current_game_stage, _list_a, _state) = response.as_game_response();
        game_stage = current_game_stage;
    } else {
        let response = sender.start(player_key, 1, 1, 1, 1).await?;
        let (current_game_stage, _list_a, _state) = response.as_game_response();
        game_stage = current_game_stage;
    }

    while game_stage != 2 {
        std::thread::sleep(std::time::Duration::from_millis(500));
        let response = sender.command(player_key, Expr::Nil).await?;
        let (current_game_stage, list_a, state) = response.as_game_response();
        game_stage = current_game_stage;
        info!("GAME STAGE:{}", game_stage);
        info!("List A:{}", list_a);
        info!("State:{}", state);
        println!("\n{}\nx", "=".repeat(50));
    }
    Ok(())
}
