use base64::{
    engine::general_purpose::URL_SAFE_NO_PAD,
    prelude::{Engine as _, BASE64_STANDARD},
};
use serde::{Deserialize, Serialize};
use std::{fs, str::FromStr};
use url::form_urlencoded;
mod errors;
use errors::Oauth2Error;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct ProviderConfig {
    pub provider: Provider,
    pub scope: String,
    pub authorization_url: String,
    pub token_exchange_url: String,
    pub app_id: String,
    pub app_secret: String,
    pub op_auth_string: String,
    pub op: String,
}

#[derive(Deserialize, Serialize, Copy, Clone, PartialEq, Eq, Debug)]
pub enum Provider {
    Github,
    Gitlab,
    Google,
    Apple,
    Okta,
    Facebook,
    Azure,
    Auth0,
    Custom,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    aud: String,
    sub: String,
    name: String,
    email: String,
    exp: u64,
}

impl FromStr for Provider {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "github" => Ok(Provider::Github),
            "gitlab" => Ok(Provider::Gitlab),
            "google" => Ok(Provider::Google),
            "apple" => Ok(Provider::Apple),
            "okta" => Ok(Provider::Okta),
            "facebook" => Ok(Provider::Facebook),
            "azure" => Ok(Provider::Azure),
            "auth0" => Ok(Provider::Auth0),
            "custom" => Ok(Provider::Custom),
            _ => Err(()),
        }
    }
}

impl Into<String> for Provider {
    fn into(self) -> String {
        match self {
            Provider::Github => "Github".to_string(),
            Provider::Gitlab => "Gitlab".to_string(),
            Provider::Google => "Google".to_string(),
            Provider::Apple => "Apple".to_string(),
            Provider::Okta => "Okta".to_string(),
            Provider::Facebook => "Facebook".to_string(),
            Provider::Azure => "Azure".to_string(),
            Provider::Auth0 => "Auth0".to_string(),
            Provider::Custom => "Custom".to_string(),
        }
    }
}
#[derive(Deserialize, Serialize)]
pub struct Config {
    pub provider: Vec<ProviderConfig>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u64,
    pub id_token: Option<String>,
    pub refresh_token: Option<String>,
}

/// Get the name of the provider config file
/// from the OAUTH2_CONFIG_FILE environment variable or
/// default to "oauth2.toml"
///
/// # Returns
/// The name of the provider config file
pub fn get_provider_config_file() -> String {
    std::env::var("OAUTH2_CONFIG_FILE").unwrap_or_else(|_| "oauth2.toml".to_string())
}

/// Get the redirect url for the specified provider
///
/// # Arguments
/// * `provider_config` - The provider configuration
///
/// # Returns
/// The redirect url
///
pub fn get_provider_config(config_file: &str) -> Vec<ProviderConfig> {
    let config_file_content = fs::read_to_string(config_file).expect("Failed to read config file");
    let config: Config = toml::from_str(&config_file_content).expect("Failed to parse config file");
    config.provider
}

/// Get redirect url for the provider
///
/// # Arguments
/// * `provider_config` - The provider configuration
/// * `callback_url` - The callback url
/// * `state` - The state (for CSRF protection)
///
/// # Returns
/// The redirect url
pub fn get_redirect_url(
    provider_config: &ProviderConfig,
    callback_url: &str,
    state: &str,
) -> String {
    let redirect_url = form_urlencoded::byte_serialize(callback_url.as_bytes()).collect::<String>();
    let scope =
        form_urlencoded::byte_serialize(provider_config.scope.as_bytes()).collect::<String>();
    let state = form_urlencoded::byte_serialize(state.as_bytes()).collect::<String>();

    format!(
        "{}?client_id={}&redirect_uri={}&response_type=code&scope={}&state={}",
        provider_config.authorization_url, provider_config.app_id, redirect_url, scope, state
    )
}

/// Get the authorization header for the provider
///
/// # Arguments
/// * `provider_config` - The provider configuration
///
/// # Returns
/// The authorization header
pub fn get_authorization_header(provider_config: &ProviderConfig) -> String {
    format!(
        "Basic {}",
        BASE64_STANDARD.encode(format!(
            "{}:{}",
            provider_config.app_id, provider_config.app_secret
        ))
    )
}

/// Exchange the code for an access token
///
/// # Arguments
/// * `provider_config` - The provider configuration
/// * `code` - The code
/// * `callback_url` - The callback url
///
/// # Returns
/// 
/// * The access token
/// * The username
/// * The email
pub async fn exchange_code(
    provider_config: &ProviderConfig,
    code: &str,
    callback_url: &str,
) -> Result<(String,String,String), Oauth2Error> {
    let code = form_urlencoded::byte_serialize(code.as_bytes()).collect::<String>();
    //let callback_url = form_urlencoded::byte_serialize(callback_url.as_bytes()).collect::<String>();
    let authorization_header = get_authorization_header(provider_config);
    let response = reqwest::Client::new()
        .post(provider_config.token_exchange_url.as_str())
        .header("Authorization", authorization_header)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .form(&[
            ("grant_type", "authorization_code"),
            ("code", code.as_str()),
            ("redirect_uri", callback_url),
            ("client_id", provider_config.app_id.as_str()),
        ])
        .send()
        .await
        .map_err(|_| Oauth2Error::ExchangeCodeError)?;

    let response = response
        .error_for_status()
        .map_err(|_| Oauth2Error::ExchangeCodeError)?;

    let body = response
        .json::<TokenResponse>()
        .await
        .map_err(|_| Oauth2Error::ExchangeCodeError)?;

    if body.id_token.is_some() {
        // Response contains an id_token which is a JWT token
        let id_token = body.id_token.unwrap();
        let resp = decode_id_token(&id_token);
        if resp.is_err() {
            return Err(Oauth2Error::DecodeIdTokenError);
        }
        let (name, email) = resp.unwrap();
        return Ok((body.access_token, name, email));
    }
    Ok((body.access_token, "".to_string(), "tobefilled@example.org".to_string()))
}

/// Decode the id token

/// # Arguments
/// * `id_token` - The jwt id token
///
/// # Returns
/// the username and email

pub fn decode_id_token(id_token: &str) -> Result<(String, String), Oauth2Error> {
    let parts: Vec<&str> = id_token.split('.').collect();
    let claims = URL_SAFE_NO_PAD
        .decode(parts[1])
        .map_err(|_| Oauth2Error::DecodeIdTokenError)?;
    let claims: Claims =
        serde_json::from_slice(&claims).map_err(|_| Oauth2Error::DecodeIdTokenError)?;
    Ok((claims.name, claims.email))
}

/// Verify the access token
///
/// # Arguments
/// * `provider_config` - The provider configuration
/// * `access_token` - The access token
///
/// # Returns
/// (access_token, username, email)
/// The access token if valid, otherwise an error
pub async fn verify_access_token(
    provider_config: &ProviderConfig,
    access_token: &str,
) -> Result<String, Oauth2Error> {
    let client = reqwest::Client::new();
    match provider_config.provider {
        Provider::Github => {
            let response = client
                .get("https://api.github.com/user")
                .bearer_auth(access_token)
                .send()
                .await;
            match response {
                Ok(response) if response.status().is_success() => Ok(access_token.to_string()),
                _ => Err(Oauth2Error::VerifyTokenError),
            }
        }
        Provider::Google => {
            let response = client
                .get(&format!(
                    "https://www.googleapis.com/oauth2/v1/tokeninfo?access_token={}",
                    access_token
                ))
                .send()
                .await;

            match response {
                Ok(response) if response.status().is_success() => Ok(access_token.to_string()),
                _ => Err(Oauth2Error::VerifyTokenError),
            }
        }
        Provider::Facebook => {
            let app_token = format!("{}|{}", provider_config.app_id, provider_config.app_secret);
            let response = client
                .get(&format!(
                    "https://graph.facebook.com/debug_token?input_token={}&access_token={}",
                    access_token, app_token
                ))
                .send()
                .await;

            match response {
                Ok(response) if response.status().is_success() => Ok(access_token.to_string()),
                _ => Err(Oauth2Error::VerifyTokenError),
            }
        }
        Provider::Gitlab => {
            let response = client
                .post("https://gitlab.com/oauth/token/info")
                .bearer_auth(access_token)
                .send()
                .await;

            match response {
                Ok(response) if response.status().is_success() => Ok(access_token.to_string()),
                _ => Err(Oauth2Error::VerifyTokenError),
            }
        }
        // TODO: Add other providers
        Provider::Custom => Ok(access_token.to_string()),
        _ => Err(Oauth2Error::VerifyTokenError),
    }
}

/// Decode the access token
///
/// # Arguments
/// * `access_token` - The access token
///
/// # Returns
/// The decoded access token
pub fn decode_access_token(access_token: &str) -> Result<String, Oauth2Error> {
    Ok(access_token.to_string())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_provider_config_one_provider() {
        let config = r#"
            [[provider]]
            provider = "Github"
            authorization_url = "https://github.com/login/oauth/authorize"
            token_exchange_url = "https://github.com/login/oauth/access_token"
            app_id = "your_github_app_id"
            app_secret = "your_github_app_secret"
            scope = "public_profile"
            op_auth_string = "oidc/facebook"
            op = "facebook"
        "#;

        let config_file = tempfile::NamedTempFile::new().unwrap();
        std::fs::write(config_file.path(), config).unwrap();

        let providers = get_provider_config(config_file.path().to_str().unwrap());
        assert_eq!(providers.len(), 1);
        assert_eq!(providers[0].provider, Provider::Github);
        assert_eq!(
            providers[0].authorization_url,
            "https://github.com/login/oauth/authorize"
        );
        assert_eq!(
            providers[0].token_exchange_url,
            "https://github.com/login/oauth/access_token"
        );
        assert_eq!(providers[0].app_id, "your_github_app_id");
        assert_eq!(providers[0].app_secret, "your_github_app_secret");
    }

    #[test]
    fn test_get_provider_config_two_providers() {
        let config = r#"
            [[provider]]
            provider = "Github"
            authorization_url = "https://github.com/login/oauth/authorize"
            token_exchange_url = "https://github.com/login/oauth/access_token"
            app_id = "your_github_app_id"
            app_secret = "your_github_app_secret"
            scope = "public_profile"
            op_auth_string = "oidc/facebook"
            op = "facebook"

            [[provider]]
            provider = "Gitlab"
            authorization_url = "https://gitlab.com/oauth/authorize"
            token_exchange_url = "https://gitlab.com/oauth/token"
            app_id = "your_gitlab_app_id"
            app_secret = "your_gitlab_app_secret"
            scope = "public_profile"
            op_auth_string = "oidc/facebook"
            op = "facebook"
        "#;

        let config_file = tempfile::NamedTempFile::new().unwrap();
        std::fs::write(config_file.path(), config).unwrap();

        let providers = get_provider_config(config_file.path().to_str().unwrap());
        assert_eq!(providers.len(), 2);
    }

    #[test]
    fn test_decode_id_token() {
        let id_token = "eyJhbGciOiJSUzI1NiIsImtpZCI6IjhiMjFkMTM0NjExZDQxNWJkMWU2MjUzOGE0ZGRjOTA4NmYxYTZiMjUifQ.eyJpc3MiOiJodHRwczovL2RleC1tb2NrLXNlcnZlci5OT05FL2RleCIsInN1YiI6IkNpUXdPR0U0TmpnMFlpMWtZamc0TFRSaU56TXRPVEJoT1MwelkyUXhOall4WmpVME5qWVNCV3h2WTJGcyIsImF1ZCI6InNjdGdkZXNrLWFwaS1zZXJ2ZXIiLCJleHAiOjE3MTU2NzEwODQsImlhdCI6MTcxNTU4NDY4NCwiYXRfaGFzaCI6IjVvZEdyU3VrMW9lejJkc1NaRXZFM0EiLCJjX2hhc2giOiJfdFZfZFNiU09qTVVmRVdMeVVNSTNnIiwiZW1haWwiOiJhZG1pbkBkZXNrLk5PTkUiLCJlbWFpbF92ZXJpZmllZCI6dHJ1ZSwibmFtZSI6ImFkbWluIn0.AqOiwBKq2i_AoJcbfxuaVY54PN3GJjnHIn3E2FWoZY2IOu8qxvZevcUb4mjnoUZGf2QaabIcTAIxIg-mpFTRxheOPiQ1c9VSZ0vd-wNGrAG12vdraRq0-evqmFduR2G9k20QMIV8iHiGM7l93k8Fw5_bnTQId044BjepayS98bpUclS4RIIGoOLBM5IenfBCqLhHHv6oYUM6HDU4rCD02U9_Bu597wedeLdYYa7lzBDyb88ab83-eALsDpbFZ90rUnvAhpTQcl9_t51Etx-sP1yWSQ3UZ-QL61cKreqWlMbimM43R4boUWnpQTMF7ZO0EftVixEfaIQvWDRm-TLl8A";
        let (name, email) = decode_id_token(id_token).unwrap();
        assert_eq!(name, "admin");
    }
}
