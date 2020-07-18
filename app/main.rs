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
const DEFAULT_PLAYER_KEY: &str = "11111";

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
    sample(&server_url, &player_key).await?;
    let response = send("1101000".to_string()).await?;//((1,2))
    print!("{}", response);
    Ok(())
}
