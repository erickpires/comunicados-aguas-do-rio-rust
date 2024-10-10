use std::{borrow::Cow, fmt::Write};

use chrono::NaiveDate;
use lazy_static::lazy_static;
use regex::Regex;
use sha1::{Digest, Sha1};

lazy_static! {
    static ref LINE_BREAK_RE: Regex = Regex::new(r"(\r?\n)+").unwrap();
}

#[derive(Debug, Clone)]
pub struct NewsPost {
    id: String,
    
    title: String,
    url: String,
    content: String,
    date: Option<NaiveDate>,
}

impl NewsPost {
    pub fn new(title: String, url: String, content: String, date: Option<NaiveDate>) -> Self {
        Self {
            id: sha1_digest(&content),

            title,
            url,
            content,
            date
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn date(&self) -> &Option<NaiveDate>  {
        &self.date
    }

    pub fn as_markdown_string(&self) -> String {
        let date_str = self.date.map(|d| d.format("%d/%m/%Y").to_string()).unwrap_or("-".to_string());

        let mut ans = String::new();
        write!(&mut ans, "[{}]({})\n\n", self.title, self.url).expect("Unexpected error formating post");
        write!(&mut ans, "_Data: {}_\n\n", date_str).expect("Unexpected error formating post");

        ans.push_str(self.formated_content().as_ref());
        
        ans
    }

    fn formated_content(&self) -> Cow<'_, str> {
        let trimmed_content = self.content.trim();
        let ans = LINE_BREAK_RE.replace_all(trimmed_content, "\n\n");

        ans
    }
}

fn sha1_digest(msg: &String) -> String {
    let mut hasher = Sha1::new();
    hasher.update(msg);

    encode_to_hex(&hasher.finalize())
}

fn encode_to_hex(data: &[u8]) -> String {
    let mut ans = String::with_capacity(data.len() * 2);

    for byte in data {
        write!(&mut ans, "{:02X}", byte).expect("Unexpected error writing SHA1 digest");
    }

    ans
}