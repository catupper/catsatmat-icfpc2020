use http_body::Body as _;

use hyper::{Body, Client, Method, Request, StatusCode};
use hyper_tls::HttpsConnector;
use std::process;


const DEFAULT_URL: &str = "https://icfpc2020-api.testkontur.ru";
const API_KEY: &str = "41ff8e29e5fa4596928186fcfe5bfee2";

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
pub async fn send(request_string: String) -> Result<String> {
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);
    
    let server_url = DEFAULT_URL.to_string() + "/aliens/send?apiKey=" + API_KEY;
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
                while let Some(chunk) = res.body_mut().data().await {
                    match chunk {
                        Ok(content) => println!("{:?}", content),
                        Err(why) => println!("error reading body: {:?}", why)
                    }
                }                
                process::exit(2);
            }
        },
        Err(err) => {
            println!("Unexpected server response:\n{}", err);
            process::exit(1);
        }
    }
}
