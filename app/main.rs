use http_body::Body as _;
use hyper::{Client, Request, Method, Body, StatusCode, Response};
use std::env;
use std::process;

mod lib;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

const API_KEY: &str = "41ff8e29e5fa4596928186fcfe5bfee2";

async fn print_response(response: Result<Response<Body>>)->Result<()>{
    Ok(())
}

async fn sample(server_url: &str, player_key: &str) -> Result<()> {
    let client = Client::new();
    let req = Request::builder()
        .method(Method::POST)
        .uri(server_url)
        .body(Body::from(player_key.to_string()))?;

    Ok(())
}

async fn aliens(server_url: &str, request_string: String) -> Result<String> {
    let client = Client::new();
    let req = Request::builder()
        .method(Method::POST)
        .uri(server_url.to_string() + "/aliens/send?apiKey=" + API_KEY)
        .body(Body::from(request_string))?;

    match client.request(req).await{
        Ok(mut res) => {
            match res.status() {
                StatusCode::OK => {
                    print!("Server response: ");
                    let mut response = "".to_string();
                    while let Some(chunk) = res.body_mut().data().await {
                        match chunk {
                            Ok(content) => {
                                response = response + &String::from_utf8(content.to_vec()).unwrap();                                    
                                println!("{:?}", content)
                            },
                            Err(why) => println!("error reading body: {:?}", why)
                        }
                    }
                    Ok(response)
                },
                _ => {
                    println!("Unexpected server response:");
                    println!("HTTP code: {}", res.status());
                    print!("Response body: ");
                    process::exit(2);
                }
            }
        },
        Err(err) => {
            println!("Unexpected server response:\n{}", err);
            process::exit(1);
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    let server_url = &args[1];
    let player_key = &args[2];

    println!("ServerUrl: {}; PlayerKey: {}", server_url, player_key);


    let response = aliens(server_url, "01010101".to_string()).await?;
    print!("{}", response);    
    Ok(())
}
