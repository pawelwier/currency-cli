use std::io;
use std::num::ParseFloatError;
use reqwest::{Response, Error};
use serde_json::{Map, Value};

use crate::api::{fetch_currency_rates, is_currency, is_currency_list};
use crate::utils::{print_text, process_api_result};

pub fn get_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    input
}

pub async fn handle_source_currency(input: &str) -> bool {
    let result = is_currency(&input).await;
    match result {
        true => {
            print_text("Type in the target currency code(s) (eg. 'USD' or 'DKK,SEK')");
        }
        false => { 
            print_text("Invalid currency code");
        }
    }

    result
}

pub async fn handle_target_currency(input: &str) -> bool {
    let result = is_currency_list(&input).await;
    match result {
        true => {
            print_text("Provide amount");
        }
        false => { 
            print_text("Invalid currency code(s)");
        }
    }

    result
}

pub async fn handle_amount(
    input: &str,
    source_currency: &str,
    target_currency: &str
) -> Result<f32, ParseFloatError> {
    let amount_result: Result<f32, ParseFloatError> = input.trim().parse::<f32>();
    match amount_result {
        Ok(amount) => {
            let result: Result<Response, Error> = fetch_currency_rates(&source_currency, &target_currency).await;
            let data: &Map<String, Value> = &process_api_result(result).await;
            let keys: Vec<&String> = data.keys().collect();

            let mut rates: String = format!("\n{} {}----\n", amount, source_currency);
            
            for key in keys {
                rates += &format!("{:6}: {:?}\n", key, data.get(key).unwrap().to_string().parse::<f32>().unwrap() * amount);
            };
            println!("{}", rates);
        },
        Err(_) => {
            print_text("Invalid amount");
        }
    }

    amount_result
}

pub async fn update_local_currency(code: &str) -> Result<bool, std::io::Error> {
    let result = is_currency(&code).await;
    match result {
        true => {
            std::env::set_var("LOCAL_CURRENCY", code);
        }
        false => { 
            print_text("Invalid currency code");
        }
    }

    Ok(result)
}