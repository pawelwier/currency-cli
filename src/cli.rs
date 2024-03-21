use std::collections::HashMap;

use crate::utils::get_command_list;

#[derive(Debug)]
pub enum Command {
    CommandList(String),
    Rates(String),
    AllRates(String),
    CurrencyList(String),
    LocalCurrency(String),
    Exit(String)
}

#[derive(PartialEq, Debug)]
pub enum Mode {
    Rates,
    AllRates,
    Currencies,
    Default,
    Local,
    Exit
}

#[derive(PartialEq, Debug)]
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

pub fn get_mode(command_value: Option<&Command>, invalid_msg: &String) -> (String, Mode) {
    let message: &String = &invalid_msg.to_string();
    let (msg, mode) = match command_value {
        Some(Command::CommandList(value)) => (value, Mode::Default),
        Some(Command::Rates(value)) => (value, Mode::Rates),
        Some(Command::AllRates(value)) => (value, Mode::AllRates),
        Some(Command::CurrencyList(value)) => (value, Mode::Currencies),
        Some(Command::LocalCurrency(value)) => (value, Mode::Local),
        Some(Command::Exit(value)) => (value, Mode::Exit),
        None => (message, Mode::Default)
    };

    (msg.to_string(), mode)
}

pub fn get_invalid_msg(text: &str) -> String {
    format!("Invalid command: {}Type in 'list' to view available commands.", text)
} 

pub fn parse_command_main(text: &str) -> (String, Mode) {
    let command_map: HashMap<String, Command> = HashMap::from([
        ("list".to_string(), Command::CommandList(get_command_list())),
        ("rate".to_string(), Command::Rates("Type in the source currency code (eg. 'USD')".to_string())),
        ("all".to_string(), Command::AllRates("".to_string())),
        ("info".to_string(), Command::CurrencyList("All available currencies:".to_string())),
        ("local".to_string(), Command::LocalCurrency("Insert new local currency code:".to_string())),
        ("exit".to_string(), Command::Exit("Bye bye!".to_string()))
    ]);

    let command_value: Option<&Command> = command_map.get(text.trim());

    get_mode(command_value, &get_invalid_msg(text))
}