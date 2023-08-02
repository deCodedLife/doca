use std::str::from_utf8;
use reqwest;
use serde_json::{json, Value};
use futures_util::StreamExt;

use tokio;
use reqwest::Error;

use crate::configs;

const API_URL: &str = "https://dev.docacrm.com";

pub fn init(cfg: &configs::Config) {
    let api_url = cfg.api_url.to_string();
    let cashbox = cfg.cashbox_id.to_string();
    tokio::spawn( async { request_transactions(api_url, cashbox).await } );
}

fn parse_request(req: Value) {
    println!("{:?}", req );
}

pub async fn doca_post(object: &str, command: &str, data: Value) -> Result<(), Error> {
    let mut body = json!({});
    body["object"] = object.parse().unwrap();
    body["command"] = command.parse().unwrap();
    body["data"] = data;

    let request = reqwest::Client::new()
        .post(API_URL)
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await
        .unwrap()
        .json::<Value>()
        .await?;

    println!("[i] Doca response: {:?}", request);
    Ok(())
}

pub async fn change_status(sale_id: i8) {
    let mut body = json!({});
    body[ "sale_id" ] = Value::from(sale_id);
    doca_post("atol", "confirm-transaction", body).await.unwrap();
}

pub async fn request_transactions(api_url: String, cashbox: String) -> Result<(), Error> {
    let mut stream = reqwest::Client::new()
        .post(api_url)
        .header("Content-Type", "application/json")
        .header( "Accept", "text/stream" )
        .json(&json!({
            "object": "atol",
            "command": "get-operations",
            "data": {
                "cashbox_id": cashbox
            }
        }))
        .send()
        .await
        .unwrap()
        .bytes_stream();

    while let Some(event) = stream.next().await {
        let bytes = event?;
        let text = from_utf8( &bytes ).unwrap();
        let value: Value = serde_json::from_str( text ).unwrap();
        parse_request( value );
    }
    Ok(())
}
