#[derive(Debug)]
pub enum Error<'a> {
    ConnectionError(reqwest::Error),
    ElementNotFound(&'a str),
    AttrNotFound(&'a str),
    TelegramApiError,
    DatabaseConnectionError(rusqlite::Error),
}

impl From<reqwest::Error> for Error<'_> {
    fn from(value: reqwest::Error) -> Self {
        Self::ConnectionError(value)
    }
}

impl From<telegram_bot_api::bot::APIResponseError> for Error<'_> {
    fn from(_: telegram_bot_api::bot::APIResponseError) -> Self {
        Self::TelegramApiError
    }
}

impl From<rusqlite::Error> for Error<'_> {
    fn from(value: rusqlite::Error) -> Self {
        Self::DatabaseConnectionError(value)
    }
}