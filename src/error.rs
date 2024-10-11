use std::fmt::Display;

#[derive(Debug)]
pub enum Error {
    ConnectionError(reqwest::Error),
    ElementNotFound(&'static str),
    AttrNotFound(&'static str),
    TelegramApiError(telegram_bot_api::bot::APIResponseError),
    DatabaseConnectionError(rusqlite::Error),
}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Self::ConnectionError(value)
    }
}

impl From<telegram_bot_api::bot::APIResponseError> for Error {
    fn from(value: telegram_bot_api::bot::APIResponseError) -> Self {
        Self::TelegramApiError(value)
    }
}

impl From<rusqlite::Error> for Error {
    fn from(value: rusqlite::Error) -> Self {
        Self::DatabaseConnectionError(value)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ConnectionError(error) => {
                writeln!(f, "Reqwest Connection Error: {:#?}", error)
            },
            Error::ElementNotFound(element_name) => {
                writeln!(f, "Element \"{}\" not found while parsing website", element_name)
            },
            Error::AttrNotFound(attr_name) => {
                writeln!(f, "Attribute \"{}\" not found while parsing website", attr_name)
            },
            Error::TelegramApiError(error) => {
                writeln!(f, "Telegram API Connection Error: {:#?}", error)
            },
            Error::DatabaseConnectionError(error) => {
                writeln!(f, "SQLite Connection Error: {:#?}", error)
            },
        }
    }
}