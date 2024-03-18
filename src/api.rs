use reqwest::{ Response, Error };

use dotenv;

pub async fn fetch_currency_data(
    base_currency: String,
    currencies: String
) -> Result<Response, Error> {
    // TODO: error handling
    let url = dotenv::var("API_URL").unwrap();
    let api_key = dotenv::var("API_KEY").unwrap();

    // let params = ReqParams {
    //     api_key,
    //     base_currency,
    //     currencies
    // };
    
    let params = [
        ("apikey", api_key),
        ("base_currency", base_currency),
        ("currencies", currencies)
        ];
        
        
        let url_with_params = reqwest::Url::parse_with_params(&url, params).unwrap();
        println!("{}", &url_with_params);
    let body: Result<reqwest::Response, reqwest::Error> = reqwest::get(url_with_params)
        .await;

    println!("{:?}", body);
    body
}