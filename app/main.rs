use http_body::Body as _;
use hyper::{Body, Client, Method, Request, StatusCode, client::HttpConnector};
use std::env;
use std::process;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

const API_KEY: &str = "41ff8e29e5fa4596928186fcfe5bfee2";

async fn sample(client: &Client<HttpConnector>,server_url: &str, player_key: &str) -> Result<()> {
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

async fn aliens(client: &Client<HttpConnector>, server_url: &str, request_string: String) -> Result<String> {
    let server_url = server_url.to_string() + "/aliens/send?apiKey=" + API_KEY;
    let req = Request::builder()
        .method(Method::POST)
        .uri(server_url.clone())
        .body(Body::from(request_string.clone()))?;
    
    println!("ServerUrl: {}; requestString: {}", server_url, request_string);

    match client.request(req).await {
        Ok(mut res) => match res.status() {
            StatusCode::OK => {
                print!("Server response: ");
                let mut response = "".to_string();
                while let Some(chunk) = res.body_mut().data().await {
                    match chunk {
                        Ok(content) => {
                            response = response + &String::from_utf8(content.to_vec()).unwrap();
                            println!("{:?}", content)
                        }
                        Err(why) => println!("error reading body: {:?}", why),
                    }
                }
                Ok(response)
            }
            _ => {
                println!("Unexpected server response:");
                println!("HTTP code: {}", res.status());
                print!("Response body: ");
                process::exit(2);
            }
        },
        Err(err) => {
            println!("Unexpected server response:\n{}", err);
            process::exit(1);
        }
    }
}

const DEFAULT_URL: &str = "https://icfpc2020-api.testkontur.ru";
const DEFAULT_PLAYER_KEY: &str = "11111";

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::new();
    let args: Vec<String> = env::args().collect();

    let server_url = args
        .get(1)
        .cloned()
        .unwrap_or_else(|| DEFAULT_URL.to_string());
    let player_key = args
        .get(2)
        .cloned()
        .unwrap_or_else(|| DEFAULT_PLAYER_KEY.to_string());
    sample(&client, &server_url, &player_key).await?;
    let response = aliens(&client, &server_url, "1111011000010110001000".to_string()).await?;//((1,2))
    print!("{}", response);
    Ok(())
}
