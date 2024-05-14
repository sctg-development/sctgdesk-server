use core::str;
use std::{future::Future, pin::Pin};

use serde::Deserialize;
use url::form_urlencoded;

use crate::{
    errors::Oauth2Error,
    oauth_provider::{OAuthProvider, OAuthProviderFactory, OAuthResponse},
    Provider, ProviderConfig,
};

pub struct GithubProvider {
    provider_config: ProviderConfig,
}

#[derive(Debug, Deserialize)]
pub struct GithubTokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub scope: String,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct GithubUser {
    pub login: String,
    pub id: u64,
    pub node_id: String,
    pub avatar_url: String,
    pub gravatar_id: String,
    pub url: String,
    pub html_url: String,
    pub followers_url: String,
    pub following_url: String,
    pub gists_url: String,
    pub starred_url: String,
    pub subscriptions_url: String,
    pub organizations_url: String,
    pub repos_url: String,
    pub events_url: String,
    pub received_events_url: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub site_admin: bool,
    pub name: Option<String>,
    pub company: Option<String>,
    pub blog: String,
    pub location: Option<String>,
    pub email: Option<String>,
    pub hireable: Option<String>,
    pub bio: Option<String>,
    pub twitter_username: Option<String>,
    pub public_repos: u64,
    pub public_gists: u64,
    pub followers: u64,
    pub following: u64,
    pub created_at: String,
    pub updated_at: String,
    pub private_gists: u64,
    pub total_private_repos: u64,
    pub owned_private_repos: u64,
    pub disk_usage: u64,
    pub collaborators: u64,
    pub two_factor_authentication: bool,
    pub plan: Plan,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Plan {
    pub name: String,
    pub space: u64,
    pub collaborators: u64,
    pub private_repos: u64,
}

impl OAuthProviderFactory for GithubProvider {
    fn new() -> Self {
        let provider_config = Self::get_provider_config(Provider::Github);
        Self { provider_config }
    }
}
impl OAuthProvider for GithubProvider {
    fn get_redirect_url(&self, callback_url: &str, state: &str) -> String {
        let redirect_url =
            form_urlencoded::byte_serialize(callback_url.as_bytes()).collect::<String>();
        let scope = form_urlencoded::byte_serialize(self.provider_config.scope.as_bytes())
            .collect::<String>();
        let state = form_urlencoded::byte_serialize(state.as_bytes()).collect::<String>();

        format!(
            "{}?client_id={}&redirect_uri={}&scope={}&state={}&allow_signup=true",
            self.provider_config.authorization_url,
            self.provider_config.app_id,
            redirect_url,
            scope,
            state
        )
    }

    fn exchange_code(
        &self,
        code: &str,
        callback_url: &str,
    ) -> Pin<Box<dyn Future<Output = Result<OAuthResponse, Oauth2Error>> + Send + Sync>> {
        let code = code.to_string();
        let callback_url = callback_url.to_string();
        let provider_config = self.provider_config.clone();

        Box::pin(async move {
            let code = form_urlencoded::byte_serialize(code.as_bytes()).collect::<String>();
            let response = reqwest::Client::new()
                .post(provider_config.token_exchange_url.as_str())
                .header("Content-Type", "application/x-www-form-urlencoded")
                .header("Accept", "application/json")
                .form(&[
                    ("code", code.as_str()),
                    ("redirect_uri", &callback_url),
                    ("client_id", &provider_config.app_id.as_str()),
                    ("client_secret", &provider_config.app_secret.as_str()),
                ])
                .send()
                .await
                .map_err(|_| Oauth2Error::ExchangeCodeError)?;

            let response = response
                .error_for_status()
                .map_err(|_| Oauth2Error::ExchangeCodeError)?;

            let body_text = response.text().await.map_err(|_| Oauth2Error::ExchangeCodeError)?;
            let body: GithubTokenResponse = serde_json::from_str(&body_text)
                .map_err(|_| Oauth2Error::ExchangeCodeError)?;

            // Get the user info with:
            // Authorization: Bearer OAUTH-TOKEN
            // GET https://api.github.com/user
            let response  = reqwest::Client::new()
                .get("https://api.github.com/user")
                .header("Accept", "application/json")
                .header("User-Agent", format!("SCTGDesk/{}", env!("CARGO_PKG_VERSION")))
                .header("Authorization", format!("Bearer {}", body.access_token))
                .send()
                .await
                .map_err(|_| Oauth2Error::ExchangeCodeError)?;

            let user_info_text = response.text().await.map_err(|_| Oauth2Error::ExchangeCodeError)?;
            log::debug!("User info:\n {}", user_info_text);
            let user_info: GithubUser = serde_json::from_str(&user_info_text)
                .map_err(|_| Oauth2Error::ExchangeCodeError)?;

            if true {
                Ok(OAuthResponse {
                    access_token: body.access_token,
                    username: user_info.login,
                    email: user_info.email.unwrap_or("tobefilled@world.com".to_string()),
                })
            } else {
                Err(Oauth2Error::ExchangeCodeError)
            }
        })
    }

    fn get_provider_type(&self) -> Provider {
        Provider::Github
    }
}
