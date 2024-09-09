mod scrapers;
mod news_post;
mod telegram_bot;
mod error;

use dotenv::dotenv;
use scrapers::{cedae_scraper::{self, CedaeScraper}, Scraper};
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let api_key = env::var("BOT_API_KEY").expect("Could not read BOT_API_KEY");
    let chat_id = env::var("CHAT_ID").expect("Could not read CHAT_ID");

    let cedae_scraper = CedaeScraper::new();
    let data = cedae_scraper.get_posts().await;

    // let bot = telegram_bot::TelegramBot::new(api_key, chat_id).await;

    // bot.send_message("Hello!!!2").await.expect("Failed to send message");

    println!("{:?}", data);
}
