use std::marker::PhantomData;
use rocket::{
    http::Status, outcome::try_outcome, request::{FromRequest, Outcome, Request}, State
};
use utils::{unwrap_or_return, Token, IntoToken, BearerAuthToken};
use crate::{
    SessionId, UserId, 
    state::ApiState,
};

use rocket_okapi::okapi::openapi3::{
    Object, SecurityRequirement, SecurityScheme, SecuritySchemeData,
};

use rocket_okapi::request::RequestHeaderInput;

#[derive(Debug, Clone)]
pub struct AuthenticatedUserInfo {
    pub session_id: SessionId,
    pub user_id: UserId,
    pub access_token: Token,
}

#[derive(Debug)]
pub struct AuthenticatedUser<T> {
    pub info: AuthenticatedUserInfo,
    pub _ph: PhantomData<T>,
}

#[derive(Debug)]
pub struct AuthenticatedAdmin<T> {
    pub info: AuthenticatedUserInfo,
    pub username: String,
    pub _ph: PhantomData<T>,
}


#[rocket::async_trait]
impl<'r, T> FromRequest<'r> for AuthenticatedUser<T> where T: FromRequest<'r, Error = ()> + IntoToken + Send {
    type Error = T::Error;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let access_token = try_outcome!(request.guard::<T>().await).into_token();

        let state = try_outcome!(request.guard::<&State<ApiState>>().await);
            
        let access_token_info = unwrap_or_return!(
            state
            .find_session(&access_token)
            .await
            .ok_or(Outcome::Forward(Status::Unauthorized))
        );

        let authenticated_user = AuthenticatedUser {
            info: AuthenticatedUserInfo {
                session_id: access_token_info.session_id,
                user_id: access_token_info.user_id,
                access_token,
            },
            _ph: PhantomData,
        };

        Outcome::Success(authenticated_user)
    }
}

#[rocket::async_trait]
impl<'r, T> FromRequest<'r> for AuthenticatedAdmin<T> where T: FromRequest<'r, Error = ()> + IntoToken + Send {
    type Error = T::Error;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let state = try_outcome!(request.guard::<&State<ApiState>>().await);
        let user = try_outcome!(request.guard::<AuthenticatedUser<T>>().await);

        state.with_user_info(&user.info.user_id, |user_info| -> Outcome<Self, Self::Error> {
            if !user_info.admin {
                return Outcome::Forward(Status::Unauthorized);
            }

            let authenticated_admin = AuthenticatedAdmin {
                info: user.info.clone(),
                username: user_info.username.clone(),
                _ph: PhantomData,
            };
    
            Outcome::Success(authenticated_admin)
        })
        .await
        .unwrap_or_else(|| Outcome::Forward(Status::Unauthorized))
    }
}

impl<'r> rocket_okapi::request::OpenApiFromRequest<'r> for AuthenticatedUser<BearerAuthToken> {
    fn from_request_input(
        _gen: &mut rocket_okapi::gen::OpenApiGenerator,
        _name: String,
        _required: bool,
    ) -> rocket_okapi::Result<rocket_okapi::request::RequestHeaderInput> {
                // Setup global requirement for Security scheme
                let security_scheme = SecurityScheme {
                    description: Some("Requires an API key to access, format the key as a 256 bit base64 encoded string like `Ak4DJ9IDYTpaceqBlAlK5pGJq595ERpq6haBaADg_lA`.".to_owned()),
                    data: SecuritySchemeData::Http {
                        scheme: "bearer".to_owned(),
                        bearer_format: Some("256 bit base64 encoded string".to_owned()),
                    },
                    extensions: Object::default(),
                };
                // Add the requirement for this route/endpoint
                // This can change between routes.
                let mut security_req = SecurityRequirement::new();
                // Each security requirement needs to be met before access is allowed.
                security_req.insert("authorization".to_owned(), Vec::new());
                // These vvvvvvv-----^^^^^^^^^^ values need to match exactly!
                Ok(RequestHeaderInput::Security(
                    "authorization".to_owned(),
                    security_scheme,
                    security_req,
                ))
    }
}

impl<'r> rocket_okapi::request::OpenApiFromRequest<'r> for AuthenticatedAdmin<BearerAuthToken> {
    fn from_request_input(
        _gen: &mut rocket_okapi::gen::OpenApiGenerator,
        _name: String,
        _required: bool,
    ) -> rocket_okapi::Result<rocket_okapi::request::RequestHeaderInput> {
                // Setup global requirement for Security scheme
                let security_scheme = SecurityScheme {
                    description: Some("Requires an Admin API key to access, format the key as a 256 bit base64 encoded string like `o5Zci3V8o2QIWBDrRgDNxXwrfmX3Gk3sRjY5I302dzU=`.".to_owned()),
                    data: SecuritySchemeData::Http {
                        scheme: "bearer".to_owned(),
                        bearer_format: Some("256 bit base64 encoded string".to_owned()),
                    },
                    extensions: Object::default(),
                };
                // Add the requirement for this route/endpoint
                // This can change between routes.
                let mut security_req = SecurityRequirement::new();
                // Each security requirement needs to be met before access is allowed.
                security_req.insert("authorization_admin".to_owned(), Vec::new());
                // These vvvvvvv-----^^^^^^^^^^ values need to match exactly!
                Ok(RequestHeaderInput::Security(
                    "authorization_admin".to_owned(),
                    security_scheme,
                    security_req,
                ))
    }
}
