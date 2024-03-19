use reqwest::{Response, Error};
use serde_json::{Map, Value};

pub async fn process_api_result(result: Result<Response, Error>) -> Map<String, Value> {
    let data = result.unwrap().text().await.unwrap();
    let value: Value = serde_json::from_str(&data).unwrap();
    let object_values: &Map<String, Value> = value["data"].as_object().unwrap();
    object_values.to_owned()
}

pub fn clear_terminal() -> () {
    print!("{}[2J", 27 as char);
}

pub fn get_command_list() -> String {
    "Available commands:\n
    list - show the list of all commands,\n
    rate - get exchange rate(s) for selected currency (follow further steps),\n
    all  - get the list of all exchange rates for your local currency,\n
    info - show all available currencies,\n
    exit - quit the app,\n".to_string()
}