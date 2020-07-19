use http_body::Body as _;
use hyper::{Body, Client, Method, Request, StatusCode};
use std::env;
use std::process;

use app::*;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

async fn sample(server_url: &str, player_key: &str) -> Result<()> {
    let client = Client::new();
    let req = Request::builder()
        .method(Method::POST)
        .uri(server_url)
        .body(Body::from(player_key.to_string()))?;

    println!("ServerUrl: {}; PlayerKey: {}", server_url, player_key);
    
    match client.request(req).await {
        Ok(mut res) => {
            match res.status() {
                StatusCode::OK => {
                    print!("Server response: ");
                    while let Some(chunk) = res.body_mut().data().await {
                        match chunk {
                            Ok(content) => println!("{:?}", content),
                            Err(why) => println!("error reading body: {:?}", why)
                        }
                    }
                },
                _ => {
                    println!("Unexpected server response:");
                    println!("HTTP code: {}", res.status());
                    print!("Response body: ");
                    while let Some(chunk) = res.body_mut().data().await {
                        match chunk {
                            Ok(content) => println!("{:?}", content),
                            Err(why) => println!("error reading body: {:?}", why)
                        }
                    }
                    process::exit(2);
                }
            }
        },
        Err(err) => {
            println!("Unexpected server response:\n{}", err);
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
    let args: Vec<String> = env::args().collect();

    let server_url = args
        .get(1)
        .cloned()
        .unwrap_or_else(|| DEFAULT_URL.to_string());
    let player_key = args
        .get(2)
        .cloned()
        .unwrap_or_else(|| DEFAULT_PLAYER_KEY.to_string());
    if player_key != DEFAULT_PLAYER_KEY{
        sample(&server_url, &player_key).await?;
    }
    let sender = Sender::new(server_url.to_string(), API_KEY.to_string());
    let player_key = player_key.parse()?;

    let response = sender.join(player_key).await?;
    println!("{}", response);

    let response = sender.start(player_key,0,0,0,0).await?;
    println!("{}", response);
    for _ in 0i32..10{
        let (stage_id, list_a, state) = sender.command(player_key, Expr::Nil).await?;
        println!("Stage ID:{}", stage_id);
        println!("List A:{}", list_a);
        println!("State:{}", state);
        println!("\n{}\nx", "=".repeat(50))
    }
    println!("{}", response);
    Ok(())
}
