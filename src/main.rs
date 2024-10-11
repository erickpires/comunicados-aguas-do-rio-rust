mod scrapers;
mod news_post;
mod telegram_bot;
mod error;
mod database;

use chrono::NaiveDate;
use database::Database;
use dotenv::dotenv;
use scrapers::{aguas_do_rio_scraper::AguasDoRioScraper, cedae_scraper::CedaeScraper, igua_scraper::IguaScraper, rio_saneamento_scraper::RioSaneamentoScraper, Scraper};
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let api_key = env::var("BOT_API_KEY").expect("Could not read BOT_API_KEY");
    let chat_id = env::var("CHAT_ID").expect("Could not read CHAT_ID");
    let bot = telegram_bot::TelegramBot::new(api_key, chat_id).await;

    let database = Database::new().expect("Database connection error"); // TODO: Notify owner

    let scrapers: Vec<Box<dyn Scraper>> = vec![
        Box::new(CedaeScraper::new()), 
        Box::new(RioSaneamentoScraper::new()), 
        Box::new(IguaScraper::new()),
        Box::new(AguasDoRioScraper::new()),
    ];

    for scraper in scrapers {
        let posts = scraper.get_posts().await.unwrap(); // TODO: Notify owner

        for post in posts {
            if database.post_exists(post.id()).unwrap() { // TODO: Notify owner
                continue;
            }

            bot.send_message(&post.as_markdown_string()).await.unwrap(); // TODO: Notify owner

            database.save_post(post.id(), post.date()).unwrap(); // TODO: Notify owner;
        }
    }
}
