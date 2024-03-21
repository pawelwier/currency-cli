use dotenv;
use reqwest::{Response, Error};
use serde_json::{Map, Value};

pub async fn process_api_result(result: Result<Response, Error>) -> Map<String, Value> {
    match result {
        Ok(response) => {
            let data: String = response.text().await.unwrap();
            match serde_json::from_str::<Value>(&data) {
                Ok(value) => {
                    match value["data"].as_object() {
                        Some(object_values) => object_values.to_owned(),
                        None => panic!("Invalid response format")
                    }                    
                },
                Err(e) => panic!("Error while processing the response JSON data: {:?}", e)
            }
        },
        Err(e) => panic!("Error while processing the result: {:?}", e)
    }
}

pub fn clear_terminal() -> () {
    print!("{}[2J", 27 as char);
}

pub fn get_command_list() -> String {
    "Available commands:\n
    list   - show the list of all commands,\n
    rate   - get exchange rate(s) for selected currency (follow further steps),\n
    all    - get the list of all exchange rates for your local currency,\n
    info   - show all available currencies,\n
    local  - update your local currency (follow further steps),\n
    exit   - quit the app.\n".to_string()
}

pub fn get_env_value(key: &str) -> String {
    dotenv::var(key).expect(&format!("The .env file should have corresponding value for key: {}", key))
}

pub fn print_text(text: &str) -> () {
    println!("\n{}\n", text);
}