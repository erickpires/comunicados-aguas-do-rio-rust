use telegram_bot_api::{bot::{self, BotApi}, methods::SendMessage};

pub struct TelegramBot {
    bot_api: BotApi,
    chat_id: String
}

impl TelegramBot {
    pub async fn new(token: String, chat_id: String) -> Self {
        let bot = bot::BotApi::new(token, None).await;
        
        let Ok(bot_api) = bot else {
            panic!("Failed to login");
        };

        Self { 
            bot_api,
            chat_id
        }
    }

    pub async fn send_message(&self, msg: &str) -> Result<(), Box<dyn std::error::Error>> {
        let request = SendMessage::new(telegram_bot_api::types::ChatId::StringType(self.chat_id.clone()), msg.to_string());
        self.bot_api.send_message(request).await.map(|_|())
    }
}