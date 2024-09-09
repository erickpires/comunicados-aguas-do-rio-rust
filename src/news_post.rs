use std::fmt::Write;

use chrono::NaiveDate;
use sha1::{Digest, Sha1};

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

    pub fn as_markdown_string(&self) -> String {
        let mut ans = String::new();

        write!(&mut ans, "[{}]({})\n\n", self.title, self.url).expect("Unexpected error formating post");
        if let Some(date) = &self.date {
            ans.push_str("Data: ");
            ans.push_str(&date.format("%d/%m/%Y").to_string());
            ans.push_str("\n\n");
        }

        ans.push_str(&self.content);
        
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