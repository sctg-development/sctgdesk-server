use crate::{tokens::Token, unwrap_or_return};
use rocket::{
    http::{hyper::header::AUTHORIZATION, Status},
    request::{FromRequest, Outcome, Request},
};

pub trait IntoToken {
    fn into_token(self) -> Token;
}

#[derive(Debug)]
pub struct BearerAuthToken {
    pub token: Token,
}

impl IntoToken for BearerAuthToken {
    fn into_token(self) -> Token {
        self.token
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for BearerAuthToken {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let auth_string = unwrap_or_return!(request
            .headers()
            .get_one(AUTHORIZATION.as_str())
            .ok_or(Outcome::Forward(Status::Unauthorized)));

        let (bearer, mut token_str) = unwrap_or_return!(auth_string
            .split_once(' ')
            .ok_or(Outcome::Forward(Status::Unauthorized)));

        if !bearer.eq_ignore_ascii_case("Bearer") {
            return Outcome::Forward(Status::Unauthorized);
        };

        token_str = token_str.trim();

        let token = unwrap_or_return!(
            Token::from_str(token_str).map_err(|_| Outcome::Forward(Status::Unauthorized))
        );

        let bearer = Self { token };

        Outcome::Success(bearer)
    }
}

#[derive(Debug)]
pub struct CookieAuthToken {
    pub token: Token,
}

impl IntoToken for CookieAuthToken {
    fn into_token(self) -> Token {
        self.token
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for CookieAuthToken {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let token = unwrap_or_return!(request
            .cookies()
            .get(AUTHORIZATION.as_str())
            .map(|cookie| cookie.value().trim())
            .and_then(|token_str| Token::from_str(token_str).ok())
            .ok_or(Outcome::Forward(Status::Unauthorized)));

        let cookie_auth = Self { token };

        Outcome::Success(cookie_auth)
    }
}

#[derive(Debug)]
pub struct MixedAuthToken {
    pub token: Token,
}

impl IntoToken for MixedAuthToken {
    fn into_token(self) -> Token {
        self.token
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for MixedAuthToken {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let auth_string = request.headers().get_one(AUTHORIZATION.as_str());
        let token = request
            .cookies()
            .get(AUTHORIZATION.as_str())
            .map(|cookie| cookie.value().trim())
            .and_then(|token_str| Token::from_str(token_str).ok());

        if auth_string.is_some() {
            // Client sent token in Authorization header use it
            // Take care if an Authorization header is sent and if is malformed it will fail even if a good cookie is sent
            let auth_string = auth_string.unwrap();
            let (bearer, mut token_str) = unwrap_or_return!(auth_string
                .split_once(' ')
                .ok_or(Outcome::Forward(Status::Unauthorized)));

            if !bearer.eq_ignore_ascii_case("Bearer") {
                return Outcome::Forward(Status::Unauthorized);
            };

            token_str = token_str.trim();

            let token = unwrap_or_return!(
                Token::from_str(token_str).map_err(|_| Outcome::Forward(Status::Unauthorized))
            );

            let bearer = Self { token };
            return Outcome::Success(bearer);
        }

        if token.is_some() {
            // Client sent token in cookie use it
            let token = token.unwrap();
            let cookie_auth = Self { token };

            return Outcome::Success(cookie_auth);
        }

        Outcome::Forward(Status::Unauthorized)
    }
}
