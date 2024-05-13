use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub enum Oauth2Error {
    ExchangeCodeError,
    VerifyTokenError,
    DecodeIdTokenError,
}

impl fmt::Display for Oauth2Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Oauth2Error::ExchangeCodeError => write!(f, "Exchange code error"),
            Oauth2Error::VerifyTokenError => write!(f, "Verify token error"),
            Oauth2Error::DecodeIdTokenError => write!(f, "Decode id token error"),
        }
    }
}

impl Error for Oauth2Error {}