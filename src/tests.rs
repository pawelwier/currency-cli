#[cfg(test)]

mod tests{
    use reqwest::{Error, Response};

    use crate::api::fetch_data;    
    use crate::cli::{
        get_invalid_msg, get_mode, get_rate_mode, parse_command_main, Command, Mode, RateMode
    };
    use crate::utils::get_command_list;

    /* api */
    #[tokio::test]
    async fn test_api_responses() {
        let params: [(&str, String); 2] = [
            ("base_currency", "SEK".to_string()),
            ("currencies", "EUR".to_string())
        ];

        let latest_data: Result<Response, Error> = fetch_data("/latest", &params).await;
        let currencies_data: Result<Response, Error> = fetch_data("/currencies", &[]).await;

        match latest_data {
            Ok(response) => {
                assert_eq!(response.status(), 200)
            },
            Err(_e) => {}
        }

        match currencies_data {
            Ok(response) => {
                assert_eq!(response.status(), 200)
            },
            Err(_e) => {}
        }
    }

    /* cli */
    #[test]
    fn test_get_rate_mode() {
        assert_eq!(get_rate_mode("NOK", ""), RateMode::Target);
        assert_eq!(get_rate_mode("DKK", "NOK,EUR"), RateMode::Amount);
    }

    #[test]
    fn test_get_mode() {
        assert_eq!(
            get_mode(Some(&Command::Exit("See you!".to_string())), &"All wrong!".to_string()), 
            ("See you!".to_string(), Mode::Exit)
        );
        assert_eq!(
            get_mode(Some(&Command::Rates("Please write your selected currency.".to_string())), &"All wrong!".to_string()), 
            ("Please write your selected currency.".to_string(), Mode::Rates)
        );
    }

    #[test]
    fn test_parse_command_main() {
        assert_eq!(
            parse_command_main("list"),
            (get_command_list(), Mode::Default)
        );
    }

    #[test]
    fn test_invalid_msg() {
        assert_eq!(
            get_invalid_msg("I don't belong here"),
            format!("Invalid command: {}Type in 'list' to view available commands.", "I don't belong here")
        )
    }
}