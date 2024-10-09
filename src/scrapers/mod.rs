use async_trait::async_trait;

use crate::news_post::NewsPost;
use crate::error::Error;

pub mod cedae_scraper;
pub mod rio_saneamento_scraper;
pub mod igua_scraper;
pub mod aguas_do_rio_scraper;

#[async_trait(?Send)]
pub trait Scraper {
    async fn get_posts(&self) -> Result<Vec<NewsPost>, Error>;
}