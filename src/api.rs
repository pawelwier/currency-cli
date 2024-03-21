use std::str::Split;
use reqwest::{Response, Error};
use serde_json::{Map, Value};

use crate::utils::{get_env_value, process_api_result};

pub async fn fetch_data(
    url: &str,
    params: &[(&str, String)]
) -> Result<Response, Error> {
    let base_url: String = get_env_value("API_URL");
    let api_key: String = get_env_value("API_KEY");
    
    let key_params: &[(&str, String)] = &[("apikey", api_key)];
    let params_final: Vec<(&str, String)> = [params, key_params].concat();
    let url_final: String = base_url + url;
        
    let url_with_params = reqwest::Url::parse_with_params(&url_final, params_final).unwrap();
    reqwest::get(url_with_params).await
}

pub async fn fetch_currency_rates(
    base_currency: &str,
    currencies: &str
) -> Result<Response, Error> {
    let params: [(&str, String); 2] = [
        ("base_currency", base_currency.to_string()),
        ("currencies", currencies.to_string())
    ];
        
    fetch_data("/latest", &params).await
}

pub async fn fetch_currencies() -> Result<Response, Error> {
    fetch_data("/currencies", &[]).await
}

pub async fn is_currency(text: &str) -> bool {
    let currencies_result: Result<Response, Error> = fetch_currencies().await;
    let data: &Map<String, Value> = &process_api_result(currencies_result).await;
    let keys: Vec<&String> = data.keys().collect();

    keys.contains(&&text.trim().to_string())
}

pub async fn is_currency_list(text: &str) -> bool {
    let codes: Split<'_, &str> = text.split(",");
    let mut is_ok: bool = true;

    for code in codes {
        if !is_currency(code).await {
            is_ok = false;
        }
    };

    is_ok
}

pub async fn get_all_exchange_rates() -> String {
    let local_currency: String = get_env_value("LOCAL_CURRENCY");
    let currencies_result: Result<Response, Error> = fetch_currencies().await;
    let currencies_data: &Map<String, Value> = &process_api_result(currencies_result).await;
    let keys: Vec<String> = currencies_data.keys().into_iter().map(|key| key.to_string()).collect();
    let keys_joined: String = keys.join(",");
    let result: Result<Response, Error> = fetch_currency_rates(&local_currency, &keys_joined).await;
    let data: &Map<String, Value> = &process_api_result(result).await;
    
    let mut rates: String = format!("\n1 {}\n-----\n", local_currency).to_string();
    for key in keys {
        rates += &format!("{:6}: {:?}\n", key, data.get(&key).unwrap().to_string().parse::<f32>().unwrap());
    };

    rates
}

pub async fn get_all_currencies() -> String {
    let currencies_result: Result<Response, Error> = fetch_currencies().await;
    let currencies_data: &Map<String, Value> = &process_api_result(currencies_result).await;
    let keys: Vec<String> = currencies_data.keys().into_iter().map(|key| key.to_string()).collect();
    
    let mut currencies: String = "".to_string();

    for key in keys {
        let currency = currencies_data.get(&key).unwrap();
        currencies += &format!(
            "   {:7} - {:8} - {:?}\n", 
            key, 
            currency["symbol_native"].to_string().replace("\"", ""),
            currency["name"].to_string().replace("\"", "")
        );
    }

    currencies
}