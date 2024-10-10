#[derive(Debug)]
pub enum Error<'a> {
    ConnectionError(reqwest::Error),
    ElementNotFound(&'a str),
    AttrNotFound(&'a str),
    TelegramApiError(),
}

impl From<reqwest::Error> for Error<'_> {
    fn from(value: reqwest::Error) -> Self {
        Self::ConnectionError(value)
    }
}

impl From<telegram_bot_api::bot::APIResponseError> for Error<'_> {
    fn from(_: telegram_bot_api::bot::APIResponseError) -> Self {
        Self::TelegramApiError()
    }
}