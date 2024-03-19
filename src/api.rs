use reqwest::{Response, Error};
use dotenv;
use serde_json::{Map, Value};

use crate::utils::process_api_result;

async fn fetch_data(
    url: &str,
    params: &[(&str, String)]
) -> Result<Response, Error> {
    // TODO: error handling
    let base_url = dotenv::var("API_URL").unwrap();
    let api_key = dotenv::var("API_KEY").unwrap();
    
    let key_params: &[(&str, String)] = &[("apikey", api_key)];
    let params_final = [params, key_params].concat();
    let url_final = base_url + url;
        
    let url_with_params = reqwest::Url::parse_with_params(&url_final, params_final).unwrap();
    reqwest::get(url_with_params).await
}

pub async fn fetch_currency_rates(
    base_currency: &str,
    currencies: &str
) -> Result<Response, Error> {
    let params = [
        ("base_currency", base_currency.to_string()),
        ("currencies", currencies.to_string())
    ];
        
    fetch_data("/latest", &params).await
}

pub async fn fetch_currencies(
    // currencies: &str // TODO: add filter
) -> Result<Response, Error> {
    // TODO: remove params, add as option arg 
    fetch_data("/currencies", &[]).await
}

pub async fn is_currency(text: &str) -> bool {
    let currencies_result = fetch_currencies().await;
    let data: &Map<String, Value> = &process_api_result(currencies_result).await;
    let keys: Vec<&String> = data.keys().collect();

    keys.contains(&&text.trim().to_string())
}

pub async fn is_currency_list(text: &str) -> bool {
    let codes = text.split(",");

    let mut is_ok: bool = true;

    for code in codes {
        if !is_currency(code).await {
            is_ok = false;
        }
    };

    is_ok
}