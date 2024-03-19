use api::{fetch_currency_rates, is_currency, is_currency_list};
use serde_json::{Map, Value};
use utils::{process_api_result, clear_terminal};

use std::io;
use reqwest::{Response, Error};
use commands::{parse_command_main, get_rate_mode, Mode, RateMode};

mod api;
mod commands;
mod input_output;
mod utils;

#[tokio::main]
async fn main() {
    clear_terminal();
    println!("Hi! Welcome to the currency exchange system. Type in 'list' to see all available actions.\n");
    loop {
        println!("Please insert your action.\n");

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        clear_terminal();

        let (msg, mode) = parse_command_main(&input);
        println!("\n{}\n", msg);
        
        // TODO: move out, refactor
        match mode {
            Mode::Default => continue,
            Mode::Exit => break,
            Mode::GetSingle => {
                let mut source_currency: String = String::new();
                let mut target_currency: String = String::new();
                
                clear_terminal();
                loop {
                    let mut input = String::new();
                    io::stdin().read_line(&mut input).unwrap();

                    let rate_mode = get_rate_mode(&source_currency, &target_currency);
                    match rate_mode {
                        RateMode::Source => {
                            clear_terminal();
                            let result = is_currency(&input).await;
                            match result {
                                true => {
                                    source_currency = input;
                                    println!("Insert target currency code(s) (eg. 'USD' or 'DKK,SEK')\n");
                                    continue;
                                }
                                false => { 
                                    println!("Invalid currency code\n");
                                 }
                            }
                        },
                        RateMode::Target => {
                            clear_terminal();
                            let result = is_currency_list(&input).await;
                            match result {
                                true => {
                                    target_currency = input;
                                    print!("Input: {}Output: {}\n", source_currency, target_currency);
                                    println!("Provide amount\n");
                                    continue;
                                }
                                false => { 
                                    println!("Invalid currency code(s)\n");
                                 }
                            }
                        },
                        RateMode::Amount => {
                            clear_terminal();
                            let amount_result = input.trim().parse::<f32>();
                            match amount_result {
                                Ok(amount) => {
                                    let result: Result<Response, Error> = fetch_currency_rates(&source_currency, &target_currency).await;
                                    let data: &Map<String, Value> = &process_api_result(result).await;
                                    let keys: Vec<&String> = data.keys().collect();
                                
                                    println!("");
                                    println!("{} {}----", amount, source_currency);
                                    for key in keys {
                                        println!("{:6}: {:?}", key, data.get(key).unwrap().to_string().parse::<f32>().unwrap() * amount);
                                    };
                                    println!("");

                                    break;
                                },
                                Err(_) => {
                                    println!("Invalid amount\n");
                                }
                            }
                        }

                    }
                }
            }
        }
    }
}