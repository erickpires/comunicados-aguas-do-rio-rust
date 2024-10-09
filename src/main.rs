mod scrapers;
mod news_post;
mod telegram_bot;
mod error;

use dotenv::dotenv;
use scrapers::{cedae_scraper::CedaeScraper, igua_scraper::IguaScraper, rio_saneamento_scraper::RioSaneamentoScraper, Scraper};
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let api_key = env::var("BOT_API_KEY").expect("Could not read BOT_API_KEY");
    let chat_id = env::var("CHAT_ID").expect("Could not read CHAT_ID");

    let scrapers: Vec<Box<dyn Scraper>> = vec![
        Box::new(CedaeScraper::new()), 
        Box::new(RioSaneamentoScraper::new()), 
        Box::new(IguaScraper::new()),
    ];

    for scraper in scrapers {
        let posts = scraper.get_posts().await.expect("Failed to get posts");

        for post in posts {
            println!("{} - {:?}", post.title(), post.date().map(|d| d.format("%d/%m/%Y")));
            println!("{}\n", post.url());
            println!("{}\n\n\n\n", post.content());
        }
    }

    // let bot = telegram_bot::TelegramBot::new(api_key, chat_id).await;

    // bot.send_message("Hello!!!2").await.expect("Failed to send message");
}
