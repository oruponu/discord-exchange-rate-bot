use chrono::{Local, Timelike};
use serde::{Deserialize, Serialize};
use serenity::async_trait;
use serenity::builder::{CreateEmbed, CreateMessage};
use serenity::model::gateway::Ready;
use serenity::model::id::ChannelId;
use serenity::prelude::{Client, Context, EventHandler, GatewayIntents};
use std::{path::PathBuf, str::FromStr};

#[derive(Deserialize, Debug)]
struct Config {
    token: String,
    channel_id: u64,
}

struct Handler {
    channel_id: ChannelId,
}

#[derive(Deserialize, Debug)]
struct TickerResponse {
    data: Vec<Currency>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Currency {
    symbol: String,
    bid: String,
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let now = Local::now();
        let next_hour =
            now.with_minute(0).unwrap().with_second(0).unwrap() + chrono::Duration::hours(1);
        let wait_time = next_hour - now;
        println!("次の取得まで {} 秒待機", wait_time.num_seconds());
        tokio::time::sleep(wait_time.to_std().unwrap()).await;

        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(3600));

        loop {
            interval.tick().await;

            let exchange_rate = fetch_usd_jpy_exchange_rate().await.unwrap();
            let title = "USD/JPY";
            let description = &format!("{} 円", exchange_rate.bid);

            let path_buf = PathBuf::from_str("exchange_rate.toml").unwrap();
            let toml: String = toml::to_string(&exchange_rate).unwrap();
            std::fs::write(&path_buf, toml).unwrap();

            let embed = CreateEmbed::new().title(title).description(description);
            let builder = CreateMessage::new().embed(embed);
            self.channel_id
                .send_message(&ctx.http, builder)
                .await
                .unwrap();
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path_buf = PathBuf::from_str("config.toml").unwrap();
    let config = match read_config_file(path_buf) {
        Ok(config) => config,
        Err(e) => {
            println!("設定ファイルの読み込みに失敗しました");
            return Err(e);
        }
    };

    let token = config.token;
    let channel_id = ChannelId::new(config.channel_id);
    let intents = GatewayIntents::GUILD_MESSAGES;
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler { channel_id })
        .await?;
    client.start().await?;

    Ok(())
}

fn read_config_file(path_buf: PathBuf) -> Result<Config, Box<dyn std::error::Error>> {
    let contents = match std::fs::read_to_string(path_buf) {
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
    Ok(config)
}

async fn fetch_usd_jpy_exchange_rate() -> Result<Currency, Box<dyn std::error::Error>> {
    let response = reqwest::get("https://forex-api.coin.z.com/public/v1/ticker").await?;
    let body = response.json::<TickerResponse>().await?;
    match body
        .data
        .into_iter()
        .find(|currency| currency.symbol == "USD_JPY")
    {
        Some(currency) => Ok(currency),
        None => Err("USD/JPYの為替レートを取得できませんでした".into()),
    }
}
