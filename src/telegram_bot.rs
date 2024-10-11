use telegram_bot_api::{bot::{self, BotApi}, methods::SendMessage};

use crate::error::Error;

pub struct TelegramBot {
    bot_api: BotApi,
    chat_id: String
}

impl TelegramBot {
    pub async fn new(token: String, chat_id: String) -> Self {
        let bot_api = bot::BotApi::new(token, None).await.expect("Failed to login");

        Self { 
            bot_api,
            chat_id
        }
    }

    pub async fn send_message(&self, msg: &str) -> Result<(), Error> {
        let requests = MessageSplitIterator::new(msg)
            .map(|(msg, should_insert_ellipsis)| {
                let mut text = msg.to_string();
                if should_insert_ellipsis {
                    text.push_str(" […]");
                }

                let mut request = SendMessage::new(telegram_bot_api::types::ChatId::StringType(self.chat_id.clone()), text);
                request.parse_mode = Some("Markdown".to_string());

                request
            })
            .collect::<Vec<_>>();

        // TODO: Make a stream from the Iterator and avoid this Vec.
        for request in requests {
            self.bot_api.send_message(request).await?;
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