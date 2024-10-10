use async_trait::async_trait;
use chrono::{Month, NaiveDate};
use reqwest::Url;
use scraper::{Html, Selector};

use crate::{error::Error, news_post::NewsPost};

use super::Scraper;

pub struct IguaScraper {
    base_url: Url,

    posts_wrapper_selector: Selector,
    posts_selector: Selector,
    link_selector: Selector,
    title_selector: Selector,
    date_selector: Selector,

    post_content_selector: Selector,
}

#[async_trait(?Send)]
impl Scraper for IguaScraper {
    async fn get_posts(&self) -> Result<Vec<NewsPost>, Error> {
        let data = reqwest::get(self.base_url.clone()).await?.text().await?;
        let html = Html::parse_document(&data);

        let posts_wrapper_element = html.select(&self.posts_wrapper_selector).next().ok_or(Error::ElementNotFound(".infinite-scroll"))?;

        let mut ans = vec![];
        for post_element in posts_wrapper_element.select(&self.posts_selector) {
            let link_element = post_element.select(&self.link_selector).next().ok_or(Error::ElementNotFound("a"))?;
            let title_element = post_element.select(&self.title_selector).next().ok_or(Error::ElementNotFound("h3"))?;
            let date_element = post_element.select(&self.date_selector).next().ok_or(Error::ElementNotFound("p > span > span"))?;

            let url_str = link_element.value().attr("href").ok_or(Error::AttrNotFound("href"))?;
            let date_text = date_element.text().collect::<String>();
            
            let title = title_element.text().map(str::trim).collect();
            let url = self.base_url.join(url_str).unwrap();
            let date = Self::parse_date(&date_text);
            let content = self.get_post_content(url.clone()).await?;

            ans.push(NewsPost::new(title, url.to_string(), content, date));
        }

        Ok(ans)
    }
}

impl IguaScraper {
    async fn get_post_content(&self, url: Url) -> Result<String, Error> {
        let data = reqwest::get(url).await?.text().await?;
        let html = Html::parse_document(&data);

        let content_element = html.select(&self.post_content_selector).next().ok_or(Error::ElementNotFound(".news-spotlight > div"))?;

        Ok(content_element.text().collect())
    }

    fn parse_date(date_text: &str) -> Option<NaiveDate> {
        let mut fields_iter = date_text.split("de");

        let day_text = fields_iter.next()?.trim();
        let month_text = fields_iter.next()?.trim();
        let year_text = fields_iter.next()?.trim();

        let day = day_text.parse::<u32>().ok()?;
        let month = month_text.parse::<Month>().ok()?;
        let year = year_text.parse::<i32>().ok()?;

        NaiveDate::from_ymd_opt(year, month.number_from_month(), day)
    }

    pub fn new() -> Self {
        Self {
            base_url: Url::parse("https://igua.com.br/noticias?page=1").unwrap(),

            posts_wrapper_selector: Selector::parse(".infinite-scroll").unwrap(),
            posts_selector: Selector::parse(".infinite-scroll-content").unwrap(),
            link_selector: Selector::parse("a").unwrap(),
            title_selector: Selector::parse("h3").unwrap(),
            date_selector: Selector::parse("p > span > span").unwrap(),

            post_content_selector: Selector::parse(".news-spotlight > div").unwrap(),
        }
    }
}