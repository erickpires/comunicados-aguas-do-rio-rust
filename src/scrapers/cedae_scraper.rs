use async_trait::async_trait;
use chrono::NaiveDate;
use reqwest::Url;
use scraper::{selectable::Selectable, Html, Selector};

use crate::{error::Error, news_post::NewsPost};

use super::Scraper;

pub struct CedaeScraper {
    base_url: Url,

    news_list_selector: Selector,
    links_selector: Selector,
    date_element_selector: Selector,
    content_element_selector: Selector,
}

#[async_trait(?Send)]
impl Scraper for CedaeScraper {
    async fn get_posts(&self) -> Result<Vec<NewsPost>, Error> {
        let data = reqwest::get(self.base_url.clone()).await?.text().await?;
        let html = Html::parse_document(&data);

        let news_posts_wrapper_element = html.select(&self.news_list_selector).next().ok_or(Error::ElementNotFound(".lista-busca"))?;

        let mut ans = Vec::new();
        for news_post_element in news_posts_wrapper_element.select(&self.links_selector) {
            let post_title = news_post_element.text().map(str::trim).collect();
            let post_url = news_post_element.value().attr("href").ok_or(Error::AttrNotFound("href"))?;

            let post = self.get_post_date_and_content(post_title, post_url).await?;
            ans.push(post);
        }

        Ok(ans)
    }
}

impl CedaeScraper {
    async fn get_post_date_and_content(&self, title: String, post_url: &str) -> Result<NewsPost, Error> {
        let url = self.base_url.join(post_url).unwrap();
        let post_data = reqwest::get(url.clone()).await?.text().await?;

        let html = Html::parse_document(&post_data);
        let date_element = html.select(&self.date_element_selector).next().ok_or(Error::ElementNotFound("[id$=DateStart]"))?;
        let content_element = html.select(&self.content_element_selector).next().ok_or(Error::ElementNotFound("[id$=NewsBody]"))?;

        let date_text = date_element.text().collect::<String>();
        let content_text = content_element.text().collect();

        let date = NaiveDate::parse_from_str(&date_text, "%d/%m/%Y").ok();

        Ok(NewsPost::new(title, url.to_string(), content_text, date))
    }

    pub fn new() -> Self {
        Self {
            base_url: Url::parse("https://cedae.com.br/Noticias/").unwrap(),

            news_list_selector: Selector::parse(".lista-busca").unwrap(),
            links_selector: Selector::parse("a").unwrap(),
            date_element_selector: Selector::parse("[id$=DateStart]").unwrap(),
            content_element_selector: Selector::parse("[id$=NewsBody]").unwrap(),
        }
    }
}