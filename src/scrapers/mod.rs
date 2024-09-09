use crate::news_post::NewsPost;
use crate::error::Error;

pub mod cedae_scraper;

pub trait Scraper {
    async fn get_posts(&self) -> Result<Vec<NewsPost>, Error>;
}