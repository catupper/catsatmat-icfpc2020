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
    let role = if list_a.cdr().car() == Expr::Int(1) {
        1
    } else {
        0
    };
    info!("IsDefneder: {}", role);
    assert_eq!(game_stage, 0);
    let response = sender.start(player_key, 256, 1, 1, 1).await?;
    let (current_game_stage, _list_a, state) = response.as_game_response();
    game_stage = current_game_stage;
    let mut state: State = state.into();
    let my_ship = state.ships.iter().find(|&ship| ship.role == role).unwrap();
    let ship_id = my_ship.ship_id;
    while game_stage != 2 {
        let my_ship = state.ships.iter().find(|&ship| ship.role == role).unwrap();
        let (x, y) = my_ship.position;
        //        let v = if state.turn <= 2 { 2 } else { 1 };
        let v = 1;
        let commands =
            vec![Command::accelerate(ship_id, (v * -x / x.abs(), v * -y / y.abs())).into()];
        //let commands = vec![Command::shoot(ship_id, (1, 2)).into()];
        let response = sender
            .command(player_key, Expr::from_vector(commands))
            .await?;
        let (current_game_stage, list_a, tmp_state) = response.as_game_response();
        game_stage = current_game_stage;
        info!("GAME STAGE:{}", game_stage);
        info!("List A:{}", list_a);
        info!("State:{}", tmp_state);
        state = tmp_state.into();
        info!("\n\n\n{}\n\n", "=".repeat(50));
    }
    Ok(())
}
