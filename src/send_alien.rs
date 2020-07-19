use crate::Expr;

use http_body::Body as _;
use hyper::{Body, Client, Method, Request, StatusCode};
use hyper_tls::HttpsConnector;
use log::{debug, error, info};

use std::process;

pub struct Sender {
    url: String,
    api_key: String,
}

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

impl Sender {
    pub fn new(url: String, api_key: String) -> Self {
        Sender { url, api_key }
    }

    pub async fn join_with_list(&self, player_key: i64, list: Expr) -> Result<Expr> {
        info!("JOIN");
        let expr = Expr::from_vector(vec![Expr::Int(2), Expr::Int(player_key), list]);
        self.send_expr(expr).await
    }

    pub async fn join(&self, player_key: i64) -> Result<Expr> {
        self.join_with_list(player_key, Expr::Nil).await
    }

    pub async fn start(
        &self,
        player_key: i64,
        fuel: i64,
        num2: i64,
        num3: i64,
        num4: i64,
    ) -> Result<Expr> {
        info!("START!");
        let expr = Expr::from_vector(vec![
            Expr::Int(3),
            Expr::Int(player_key),
            Expr::from_vector(vec![
                Expr::Int(fuel),
                Expr::Int(num2),
                Expr::Int(num3),
                Expr::Int(num4),
            ]),
        ]);

        self.send_expr(expr).await
    }

    pub async fn command(&self, player_key: i64, commands: Expr) -> Result<Expr> {
        info!("COMMAND!");
        let expr = Expr::from_vector(vec![Expr::Int(4), Expr::Int(player_key), commands]);
        self.send_expr(expr).await
    }

    pub async fn send_expr(&self, expr: Expr) -> Result<Expr> {
        info!("Sending Expr: {}", expr);
        let src = expr.modulate();
        let res = self.send(src).await?;
        let res_expr = Expr::demodulate(&res).0;
        info!("Receiving Expr: {}", res_expr);
        Ok(res_expr)
    }

    pub async fn send(&self, request_string: String) -> Result<String> {
        let https = HttpsConnector::new();
        let client = Client::builder().build::<_, hyper::Body>(https);

        let server_url = self.url.clone() + "/aliens/send?apiKey=" + &self.api_key;
        let req = Request::builder()
            .method(Method::POST)
            .uri(server_url.clone())
            .body(Body::from(request_string.clone()))?;

        debug!(
            "ServerUrl: {}; requestString: {}",
            server_url, request_string
        );

        match client.request(req).await {
            Ok(mut res) => match res.status() {
                StatusCode::OK => {
                    debug!("Server response: ");
                    let mut response = "".to_string();
                    while let Some(chunk) = res.body_mut().data().await {
                        match chunk {
                            Ok(content) => {
                                response = response + &String::from_utf8(content.to_vec()).unwrap();
                                debug!("{:?}", content)
                            }
                            Err(why) => error!("error reading body: {:?}", why),
                        }
                    }
                    Ok(response)
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
    }
}
