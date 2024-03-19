use std::collections::HashMap;

use crate::api::{get_all_exchange_rates, get_all_currencies};
use crate::utils::get_command_list;

#[derive(Debug)]
pub enum Command {
    CommandList(String),
    Rates(String),
    AllRates(String),
    CurrencyList(String),
    Exit(String)
}

#[derive(PartialEq)]
pub enum Mode {
    Default,
    GetSingle,
    Exit
}

#[derive(PartialEq)]
pub enum RateMode {
    Source,
    Target,
    Amount
}

pub fn get_rate_mode(
    source_currency: &str, 
    target_currency: &str
) -> RateMode {
    let provided_values: (bool, bool) = (!source_currency.is_empty(), !target_currency.is_empty());

    match provided_values {
        (true, true) => RateMode::Amount,
        (true, false) => RateMode::Target,
        (false, false) => RateMode::Source,
        (false, true) => RateMode::Source // not possible
    }
}

pub async fn parse_command_main(text: &str) -> (String, Mode) {
    let command_map: HashMap<String, Command> = HashMap::from([
        ("list".to_string(), Command::CommandList(get_command_list())),
        ("rate".to_string(), Command::Rates("Insert source currency code (eg. 'USD')".to_string())),
        ("all".to_string(), Command::AllRates(get_all_exchange_rates().await)),
        ("info".to_string(), Command::CurrencyList(get_all_currencies().await)),
        ("exit".to_string(), Command::Exit("Bye bye!".to_string()))
    ]);

    let command_value = command_map.get(text.trim());
    let invalid_msg = format!("Invalid command: {}Type in 'list' to view available commands.", text);

    let (msg, mode) = match command_value {
        Some(Command::CommandList(value)) => (value, Mode::Default),
        Some(Command::Rates(value)) => (value, Mode::GetSingle),
        Some(Command::AllRates(value)) => (value, Mode::Default),
        Some(Command::CurrencyList(value)) => (value, Mode::Default),
        Some(Command::Exit(value)) => (value, Mode::Exit),
        None => (&invalid_msg, Mode::Default)
    };

    (msg.to_string(), mode)
}