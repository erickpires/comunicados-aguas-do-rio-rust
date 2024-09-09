#[derive(Debug)]
pub enum Error<'a> {
    ConnectionError(reqwest::Error),
    ElementNotFound(&'a str),
    AttrNotFound(&'a str)
}

impl<'a> From<reqwest::Error> for Error<'a> {
    fn from(value: reqwest::Error) -> Self {
        Error::ConnectionError(value)
    }
}