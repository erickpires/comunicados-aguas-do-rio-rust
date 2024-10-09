use async_trait::async_trait;
use chrono::NaiveDate;
use reqwest::Url;
use scraper::{Html, Selector};
use serde::Deserialize;

use crate::{error::Error, news_post::NewsPost};

use super::Scraper;

#[derive(Deserialize)]
struct ApiResponse {
    html: String,
}

pub struct AguasDoRioScraper {
    base_url: Url,

    posts_selector: Selector,
    title_selector: Selector,
    date_selector: Selector,
    link_selector: Selector,
    content_selector: Selector,

    full_content_selector:Selector,
}

#[async_trait(?Send)]
impl Scraper for AguasDoRioScraper {
    async fn get_posts(&self) -> Result<Vec<NewsPost>, Error> {
        let api_reponse = reqwest::get(self.base_url.clone()).await?.json::<ApiResponse>().await?;
        let html = Html::parse_fragment(&api_reponse.html);

        let mut ans = vec![];
        for post_element in html.select(&self.posts_selector) {
            let title_element = post_element.select(&self.title_selector).next().ok_or(Error::ElementNotFound(".card-title"))?;
            let date_element = post_element.select(&self.date_selector).next().ok_or(Error::ElementNotFound(".date"))?;
            let content_element = post_element.select(&self.content_selector).next().ok_or(Error::ElementNotFound(".card-text"))?;
            let link_element = post_element.select(&self.link_selector).next().ok_or(Error::ElementNotFound(".link-title"))?;

            let title = title_element.text().map(str::trim).collect();
            let date_text = date_element.text().collect::<String>();
            let mut content = content_element.text().map(str::trim).collect::<String>();

            let link_str = link_element.value().attr("href").ok_or(Error::AttrNotFound("href"))?;
            let url = self.base_url.join(link_str).unwrap();

            let date = NaiveDate::parse_from_str(date_text.trim(), "%d/%m/%Y").ok();

            if content.ends_with("...") {
                content = self.get_full_content(url.clone()).await?;
            }

            ans.push(NewsPost::new(title, url.to_string(), content, date));
        }


        Ok(ans)
    }
}

impl AguasDoRioScraper {
    async fn get_full_content(&self, url: Url) -> Result<String, Error> {
        let data = reqwest::get(url).await?.text().await?;
        let html = Html::parse_document(&data);

        let content_element = html.select(&self.full_content_selector).next().ok_or(Error::ElementNotFound(".article-inline-text"))?;

        let content = content_element.text().map(str::trim).collect();
        
        Ok(content)
    }

    pub fn new() -> Self {
        Self {
            base_url: Url::parse("https://aguasdorio.com.br/wp-admin/admin-ajax.php?id=lista-noticias&posts_per_page=10&page=0&offset=0&repeater=default&preloaded=false&preloaded_amount=0&category=comunicados&order=DESC&orderby=date&action=alm_get_posts").unwrap(),

            posts_selector: Selector::parse(".content-holder").unwrap(),
            title_selector: Selector::parse(".card-title").unwrap(),
            date_selector: Selector::parse(".date").unwrap(),
            link_selector: Selector::parse(".link-title").unwrap(),
            content_selector: Selector::parse(".card-text").unwrap(),

            full_content_selector: Selector::parse(".article-inline-text").unwrap(),
        }
    }
}