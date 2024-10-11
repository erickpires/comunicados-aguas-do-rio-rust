use std::time::Duration;

use telegram_bot_api::{bot::{self, BotApi}, methods::SendMessage};
use tokio::time::sleep;

use crate::error::Error;

pub enum TelegramParseMode {
    Markdown,
    PlainText
}

impl TelegramParseMode {
    fn get_value(&self) -> Option<String> {
        match self {
            TelegramParseMode::Markdown => Some("Markdown".to_string()),
            TelegramParseMode::PlainText => None,
        }
    }
}

pub struct TelegramBot {
    bot_api: BotApi
}

impl TelegramBot {
    pub async fn new(token: String) -> Self {
        let bot_api = bot::BotApi::new(token, None).await.expect("Failed to login");

        Self { 
            bot_api
        }
    }

    pub async fn send_message(&self, msg: &str, chat_id: &str, parse_mode: TelegramParseMode) -> Result<(), Error> {
        const MESSAGES_INTERVAL: u64 = 3000;

        let requests = MessageSplitIterator::new(msg)
            .map(|(msg, should_insert_ellipsis)| {
                let mut text = msg.to_string();
                if should_insert_ellipsis {
                    text.push_str(" […]");
                }

                let mut request = SendMessage::new(telegram_bot_api::types::ChatId::StringType(chat_id.to_string()), text);
                request.parse_mode = parse_mode.get_value();

                request
            })
            .collect::<Vec<_>>();

        // TODO: Make a stream from the Iterator and avoid this Vec.
        for request in requests {
            self.bot_api.send_message(request).await?;
            sleep(Duration::from_millis(MESSAGES_INTERVAL)).await;
        }

        Ok(())
    }
}

struct MessageSplitIterator<'a> {
    msg: &'a str,
}

impl<'a> Iterator for MessageSplitIterator<'a> {
    type Item = (&'a str, bool);

    fn next(&mut self) -> Option<Self::Item> {
        // NOTE: Telegram limit is 4096 characters. We are leaving some room at the end 
        // so we can add ellipsis message.
        const MESSAGE_MAX_SIZE: usize = 4000;

        if self.msg.len() == 0 {
            return None;
        }

        if self.msg.len() <= MESSAGE_MAX_SIZE {
            let ans = Some((self.msg, false));
            self.msg = "";
            return ans;
        }

        // NOTE: First we try to break the message at a paragraph boundary by
        // searching for a newline character. If we can't find a suitable 
        // split point, we try to break the message at a word boundary by 
        // searching for a space characters. If no neither case is found, 
        // we split the message at the last possible character.

        let max_slice_boundary = find_floor_char_boundary(self.msg, MESSAGE_MAX_SIZE);
        let max_slice = &self.msg[..max_slice_boundary];

        if let Some(index) = max_slice.rfind('\n') {
            let ans = Some((self.msg[..index].trim(), true));
            self.msg = &self.msg[(index+1)..].trim();
            return ans;
        }

        if let Some(index) = max_slice.rfind(' ') {
            let ans = Some((self.msg[..index].trim(), true));
            self.msg = &self.msg[(index+1)..].trim();
            return ans;
        }

        self.msg = &self.msg[max_slice_boundary..];
        
        Some((max_slice, true))
    }
}

impl<'a> MessageSplitIterator<'a> {
    fn new(msg: &'a str) -> Self {
        Self {
            msg: msg.trim()
        }
    }
}

fn find_floor_char_boundary(s: &str, mut index: usize) -> usize {
    while !s.is_char_boundary(index) {
        index -= 1;
    }

    return index;
}