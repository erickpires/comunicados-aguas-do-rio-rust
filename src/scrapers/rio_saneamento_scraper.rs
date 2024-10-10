use async_trait::async_trait;
use chrono::NaiveDate;
use reqwest::Url;
use scraper::{selectable::Selectable, Html, Selector};

use crate::{error::Error, news_post::NewsPost};

use super::Scraper;

#[derive(Debug)]
struct RioSaneamentoPost {
    title: String,
    url: Url,
    date: Option<NaiveDate>,
}

pub struct RioSaneamentoScraper {
    base_url: Url,

    main_posts_wrapper_selector: Selector,
    main_posts_selector: Selector,
    main_post_title_selector: Selector,
    main_post_date_selector: Selector,

    secondary_posts_wrapper_selector: Selector,
    secondary_post_title_selector: Selector,
    secondary_post_date_selector: Selector,
    secondary_posts_selector: Selector,

    post_content_selector: Selector,
}


#[async_trait(?Send)]
impl Scraper for RioSaneamentoScraper {
    async fn get_posts(&self) -> Result<Vec<NewsPost>, Error> {
        let data = reqwest::get(self.base_url.clone()).await?.text().await?;
        let html = Html::parse_document(&data);

        let main_posts = self.get_main_posts(&html)?;
        let secondary_posts = self.get_secondary_posts(&html)?;

        let mut ans = vec![];
        for post in main_posts.into_iter().chain(secondary_posts.into_iter()) {
            let content = self.get_post_content(post.url.clone()).await?;
            let post = NewsPost::new(post.title, post.url.to_string(), content, post.date);

            ans.push(post);
        }

        Ok(ans)
    }
}

impl RioSaneamentoScraper {
    fn get_main_posts(&self, html: &Html) -> Result<Vec<RioSaneamentoPost>, Error> {
        let main_posts_wrapper = html.select(&self.main_posts_wrapper_selector).next().ok_or(Error::ElementNotFound(".gab-newsBlockWrapper"))?;

        main_posts_wrapper
            .select(&self.main_posts_selector)
            .map(|post_element| {
                let post_url = post_element.value().attr("href").ok_or(Error::AttrNotFound("href"))?;
                let title_element = post_element.select(&self.main_post_title_selector).next().ok_or(Error::ElementNotFound(".gab-newsBlockWrapper__title"))?;
                let date_element = post_element.select(&self.main_post_date_selector).next().ok_or(Error::ElementNotFound(".gab-newsBlockWrapper__date"))?;

                let date_text = date_element.text().collect::<String>();

                Ok(RioSaneamentoPost {
                    title: title_element.text().map(str::trim).collect(),
                    url: self.base_url.join(post_url).unwrap(),
                    // NOTE: Rio + Saneamento uses single digit dates
                    date: NaiveDate::parse_from_str(date_text.trim(), "%_d/%m/%Y").ok(),
                })
            })
            .collect()
    }

    fn get_secondary_posts(&self, html: &Html) -> Result<Vec<RioSaneamentoPost>, Error> {
        let secondary_posts_wrapper = html.select(&self.secondary_posts_wrapper_selector).next().ok_or(Error::ElementNotFound(".gab-latest-posts"))?;

        secondary_posts_wrapper
            .select(&self.secondary_posts_selector)
            .map(|post_element| {
                let post_url = post_element.value().attr("href").ok_or(Error::AttrNotFound("href"))?;
                let title_element = post_element.select(&self.secondary_post_title_selector).next().ok_or(Error::ElementNotFound(".card-title"))?;
                let date_element = post_element.select(&self.secondary_post_date_selector).next().ok_or(Error::ElementNotFound(".card-date"))?;

                let date_text = date_element.text().collect::<String>();

                Ok(RioSaneamentoPost {
                    title: title_element.text().map(str::trim).collect(),
                    url: self.base_url.join(post_url).unwrap(),
                    // NOTE: Rio + Saneamento uses single digit dates
                    date: NaiveDate::parse_from_str(date_text.trim(), "%_d/%m/%Y").ok(),
                })
            })
            .collect()
    }

    async fn get_post_content(&self, url: Url) -> Result<String, Error> {
        let data = reqwest::get(url).await?.text().await?;
        let html = Html::parse_document(&data);

        let content_element = html.select(&self.post_content_selector).next().ok_or(Error::ElementNotFound(".content-single__content"))?;

        Ok(content_element.text().collect())
    }

    pub fn new() -> Self {
        Self {
            base_url: Url::parse("https://www.riomaissaneamento.com.br/noticias/").unwrap(),

            main_posts_wrapper_selector: Selector::parse(".gab-newsBlockWrapper").unwrap(),
            main_posts_selector: Selector::parse("a").unwrap(),
            main_post_title_selector: Selector::parse(".gab-newsBlockWrapper__title").unwrap(),
            main_post_date_selector: Selector::parse(".gab-newsBlockWrapper__date").unwrap(),

            secondary_posts_wrapper_selector: Selector::parse(".gab-latest-posts").unwrap(),
            secondary_posts_selector: Selector::parse(".href-wrapper").unwrap(),
            secondary_post_title_selector: Selector::parse(".card-title").unwrap(),
            secondary_post_date_selector: Selector::parse(".card-date").unwrap(),

            post_content_selector: Selector::parse(".content-single__content").unwrap(),
        }
    }
}