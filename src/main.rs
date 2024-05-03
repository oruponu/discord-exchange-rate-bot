use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Config {
    token: String,
    channel_id: u64,
}

#[derive(Deserialize, Debug)]
struct TickerResponse {
    status: i32,
    data: Vec<Currency>,
    responsetime: String,
}

#[derive(Deserialize, Debug)]
struct Currency {
    symbol: String,
    ask: String,
    bid: String,
    timestamp: String,
    status: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let contents = match std::fs::read_to_string("config.toml") {
        Ok(contents) => contents,
        Err(e) => {
            return Err(e.into());
        }
    };
    let config = match toml::from_str::<Config>(&contents) {
        Ok(config) => config,
        Err(e) => {
            return Err(e.into());
        }
    };
    println!("{:?}", config);

    let response = reqwest::get("https://forex-api.coin.z.com/public/v1/ticker").await?;
    let body = response.json::<TickerResponse>().await?;
    let usd_jpy = body
        .data
        .into_iter()
        .find(|currency| currency.symbol == "USD_JPY");

    match usd_jpy {
        Some(currency) => println!("USD/JPY: {}", currency.bid),
        None => println!("米ドル／日本円の為替レートを取得できませんでした"),
    }

    Ok(())
}
