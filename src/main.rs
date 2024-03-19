use cli::{parse_command_main, get_rate_mode, Mode, RateMode};
use input_output::{get_input, handle_source_currency, handle_target_currency, handle_amount};
use utils::clear_terminal;

mod api;
mod cli;
mod input_output;
mod utils;
mod tests;

#[tokio::main]
async fn main() {
    clear_terminal();
    println!("Hi! Welcome to the currency exchange system. Type in 'list' to see all available actions.\n");
    loop {
        println!("Please type in your action.\n");

        let input: String = get_input();
        clear_terminal();

        let (msg, mode) = parse_command_main(&input).await;
        println!("\n{}\n", msg);
        
        match mode {
            Mode::Default => continue,
            Mode::Exit => break,
            Mode::GetSingle => {
                let mut source_currency: String = String::new();
                let mut target_currency: String = String::new();
                
                clear_terminal();
                loop {
                    let input: String = get_input();
                    let rate_mode = get_rate_mode(&source_currency, &target_currency);
                    match rate_mode {
                        RateMode::Source => {
                            clear_terminal();
                            if handle_source_currency(&input).await {
                                source_currency = input;
                                continue;
                            }
                        },
                        RateMode::Target => {
                            clear_terminal();
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