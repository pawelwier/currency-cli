## GETTING STARTED
1. Register here: https://app.freecurrencyapi.com/register to get your free 5k request api key
2. Clone this repo and replace the `.env.defaults` file with `.env`. Paste your api key next to `API_KEY`
3. 
- Build the app using `cargo build` OR
- Build the app using `cargo build --release` for release
4. Run your app using `cargo run`

## USING THE APP
1. Communicate with the app using a set of simple commands and confirming the choice by pressing Enter. You will be notified if you type in a wrong command.
2. The app tells you to run the `list` command to see all six available actions, that is:
  - `list` - provides a full list of commands. Two will require additional input(s).
  - `rate` - main functionality. First, you will be prompted to insert the source currency code, eg. `NOK`, then one or more target currency codes separated by commas, eg. `EUR,PLN,SEK`. Finally, type in the amount you'd like to exchange, either as an integer, eg. `1400` or a float, eg. `26.45`. A list will be displayed.
  - `all` - a list of all available currencies and their exchange rates for your local currencies will be displayed
  - `info` - a more detailed list of all available currencies, with symbols and full names will be displayed
  - `local` - you will be prompted to insert your new local currency code, eg. `USD`. The old value will be updated
  - `exit` - quit the app, head to the nearest currency exchange office and expand your wealth.

  ## ISSUES
  Docker build keeps getting an error on step `[server build 4/4]`. I'm getting a message with the path
  `"/src/target/release/build/openssl-sys-d70118955b6e7059/out/openssl-build/build/src"` missing. I've got a `openssl-sys-d1344be59675748c` folder there (different id). Tried manually changing the name but issue remained.