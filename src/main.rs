use serde::Deserialize;

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
    let response = reqwest::get("https://forex-api.coin.z.com/public/v1/ticker").await?;
    let body = response.json::<TickerResponse>().await?;
    println!("{:?}", body);
    Ok(())
}
