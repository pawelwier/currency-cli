use api::{get_all_exchange_rates, get_all_currencies};
use cli::{parse_command_main, get_rate_mode, Mode, RateMode};
use input_output::{
    get_input, handle_source_currency, handle_target_currency, handle_amount, update_local_currency
};
use utils::{clear_terminal, print_text};

mod api;
mod cli;
mod input_output;
mod utils;
mod tests;

#[tokio::main]
async fn main() {
    clear_terminal();
    print_text("Hi! Welcome to the currency exchange system. Type in 'list' to see all available actions.");
    loop {
        print_text("Please type in your action.");

        let input: String = get_input();
        clear_terminal();

        let (msg, mode) = parse_command_main(&input);
        print_text(&msg);
        
        match mode {
            Mode::Default => continue,
            Mode::Exit => break,
            Mode::AllRates => {
                print_text(&get_all_exchange_rates().await);
                continue;
            }
            Mode::Currencies => {
                print_text(&get_all_currencies().await);
                continue;
            }
            Mode::Local => {
                loop {
                    let input: String = get_input();
                    match update_local_currency(&input).await {
                        Ok(value) => { 
                            match value {
                                true => {
                                    clear_terminal();
                                    print_text(&format!("Local currency updated to {}", input));
                                    break;
                                }
                                false => {
                                    continue;
                                }
                            }
                        },
                        Err(_) => ()
                    }
                }
            }
            Mode::Rates => {
                let mut source_currency: String = String::new();
                let mut target_currency: String = String::new();
                
                loop {
                    let input: String = get_input();
                    let rate_mode = get_rate_mode(&source_currency, &target_currency);
                    match rate_mode {
                        RateMode::Source => {
                            if handle_source_currency(&input).await {
                                source_currency = input;
                                continue;
                            }
                        },
                        RateMode::Target => {
                            if handle_target_currency(&input).await {
                                target_currency = input;
                                println!("Input: {}Output: {}", source_currency, target_currency);
                                continue;
                            }
                        },
                        RateMode::Amount => {
                            clear_terminal();
                            match handle_amount(&input, &source_currency, &target_currency).await {
                                Ok(_) => { break; },
                                Err(_) => ()
                            }
                        }
                    }
                }
            }
        }
    }
}