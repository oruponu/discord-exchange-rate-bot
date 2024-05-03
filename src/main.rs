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

fn main() {
    println!("Hello, world!");
}
