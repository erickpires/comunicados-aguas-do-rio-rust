mod scrapers;
mod news_post;
mod telegram_bot;
mod error;
mod database;

use database::Database;
use dotenv::dotenv;

use error::Error;
use scrapers::{aguas_do_rio_scraper::AguasDoRioScraper, cedae_scraper::CedaeScraper, igua_scraper::IguaScraper, rio_saneamento_scraper::RioSaneamentoScraper, Scraper};
use telegram_bot::{TelegramBot, TelegramParseMode};

use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let api_key = env::var("BOT_API_KEY").expect("Could not read BOT_API_KEY");
    let chat_id = env::var("CHAT_ID").expect("Could not read CHAT_ID");
    let bot_owner_chat_id = env::var("BOT_OWNER_CHAT_ID").expect("Could not read BOT_OWNER_CHAT_ID");
    let bot = telegram_bot::TelegramBot::new(api_key).await;

    match get_posts_and_send_to_telegram(&bot, &chat_id).await {
        Ok(_) => {},
        Err(error) => {
            bot.send_message("*Error running bot:* _Comunicados Aguas do Rio_", &bot_owner_chat_id, TelegramParseMode::Markdown).await.expect("Error while handling error");
            bot.send_message(&error.to_string(), &bot_owner_chat_id, TelegramParseMode::PlainText).await.expect("Error while handling error");
        },
    }
}

async fn get_posts_and_send_to_telegram(bot: &TelegramBot, chat_id: &str) -> Result<(), Error> {
    let database = Database::new()?;

    let scrapers: Vec<Box<dyn Scraper>> = vec![
        Box::new(CedaeScraper::new()), 
        Box::new(RioSaneamentoScraper::new()), 
        Box::new(IguaScraper::new()),
        Box::new(AguasDoRioScraper::new()),
    ];

    for scraper in scrapers {
        let posts = scraper.get_posts().await?;

        for post in posts {
            if database.post_exists(post.id())? {
                continue;
            }

            bot.send_message(&post.as_markdown_string(), chat_id, TelegramParseMode::Markdown).await?;

            database.save_post(post.id(), post.date())?;
        }
    }

    Ok(())
}