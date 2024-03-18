use api::fetch_currency_data;
use cli::Cli;

use std::env::args;
use reqwest::{ Response, Error };
use serde;

mod cli;
mod api;

#[tokio::main]
async fn main() {
    // TODO: refactor
    let source_currency = args().nth(1).unwrap();
    let target_currency = args().nth(2).unwrap();
    let amount_string = args().nth(3).unwrap();

    let cli = Cli {
        source_currency,
        target_currency,
        amount: amount_string.parse::<f32>().unwrap()
    };

    let ar: Vec<String> = args().collect();

    let res: Result<Response, Error> = fetch_currency_data("EUR".to_string(), "DKK".to_string()).await;

    let data = res.unwrap().text().await.unwrap();

    println!("{}", data);
}
