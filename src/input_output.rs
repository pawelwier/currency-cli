use std::io;
use std::num::ParseFloatError;
use reqwest::{Response, Error};
use serde_json::{Map, Value};

use crate::api::{fetch_currency_rates, is_currency, is_currency_list};
use crate::utils::process_api_result;

pub fn get_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    input
}

pub async fn handle_source_currency(input: &str) -> bool {
    let result = is_currency(&input).await;
    match result {
        true => {
            println!("Insert target currency code(s) (eg. 'USD' or 'DKK,SEK')\n");
        }
        false => { 
            println!("Invalid currency code\n");
        }
    }

    result
}

pub async fn handle_target_currency(input: &str) -> bool {
    let result = is_currency_list(&input).await;
    match result {
        true => {
            println!("Provide amount\n");
        }
        false => { 
            println!("Invalid currency code(s)\n");
        }
    }

    result
}

pub async fn handle_amount(input: &str, source_currency: &str, target_currency: &str) -> Result<f32, ParseFloatError> {
    let amount_result: Result<f32, ParseFloatError> = input.trim().parse::<f32>();
    match amount_result {
        Ok(amount) => {
            let result: Result<Response, Error> = fetch_currency_rates(&source_currency, &target_currency).await;
            let data: &Map<String, Value> = &process_api_result(result).await;
            let keys: Vec<&String> = data.keys().collect();
        
            println!("\n{} {}----", amount, source_currency);
            for key in keys {
                println!("{:6}: {:?}", key, data.get(key).unwrap().to_string().parse::<f32>().unwrap() * amount);
            };
            println!("");
        },
        Err(_) => {
            println!("Invalid amount\n");
        }
    }

    amount_result
}