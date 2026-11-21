// Copyright (c) 2024 Ronan LE MEILLAT for SCTG Development
//
// This file is part of the SCTGDesk project.
//
// SCTGDesk is free software: you can redistribute it and/or modify
// it under the terms of the Affero General Public License version 3 as
// published by the Free Software Foundation.
//
// SCTGDesk is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// Affero General Public License for more details.
//
// You should have received a copy of the Affero General Public License
// along with SCTGDesk. If not, see <https://www.gnu.org/licenses/agpl-3.0.html>.
pub mod dex_provider;
pub mod github_provider;
pub mod oauth2_provider;
pub mod oauth_provider;
use serde::{Deserialize, Serialize};
use std::{fs, str::FromStr};
mod errors;

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
    /// `Custom` is the name used by the sample files (and by `FromStr`) for a Dex / generic OIDC server.
    #[serde(alias = "Custom")]
    Dex,
    Oauth2,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
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
            "custom" => Ok(Provider::Dex),
            "oauth2" => Ok(Provider::Oauth2),
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
            Provider::Dex => "Dex".to_string(),
            Provider::Oauth2 => "Oauth2".to_string(),
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

/// Get the providers config from a config file
///
/// # Returns  
/// The providers config
pub fn get_providers_config_from_file(config_file: &str) -> Vec<ProviderConfig> {
    // If file does not exist create it
    if !std::path::Path::new(&config_file).exists() {
        log::error!("oauth2 config file does not exist, creating it, we recommend you to fill it with your own values, you can change the file path by setting the OAUTH2_CONFIG_FILE environment variable.");
        let oauth2_config = include_str!("../../api-server/oauth2.toml");
        fs::write(&config_file, oauth2_config).expect("Failed to write oauth2 config file");
    }
    let config_file_content = fs::read_to_string(config_file).expect("Failed to read config file");
    let config: Config = toml::from_str(&config_file_content).expect("Failed to parse config file");
    config.provider
}

/// Get the name of the provider config file
/// from the OAUTH2_CONFIG_FILE environment variable or
/// default to "oauth2.toml"
///
/// # Returns
/// The name of the provider config file
pub fn get_providers_config_file() -> String {
    std::env::var("OAUTH2_CONFIG_FILE").unwrap_or_else(|_| "oauth2.toml".to_string())
}

/// Template shown by the console when `oauth2.toml` does not exist yet.
pub const CONFIG_TEMPLATE: &str = "# No oauth2 provider configured yet. Example:\n\
# [[provider]]\n\
# provider = \"Github\"\n\
# authorization_url = \"https://github.com/login/oauth/authorize\"\n\
# token_exchange_url = \"https://github.com/login/oauth/access_token\"\n\
# app_id = \"your_github_app_id\"\n\
# app_secret = \"your_github_app_secret\"\n\
# scope = \"read:user\"\n\
# op_auth_string = \"oidc/github\"\n\
# op = \"github\"\n";

/// Whether `value` is an absolute http(s) URL with a host.
fn is_http_url(value: &str) -> bool {
    url::Url::parse(value)
        .map(|u| matches!(u.scheme(), "http" | "https") && u.host_str().is_some())
        .unwrap_or(false)
}

/// Validates the text of an `oauth2.toml` without touching the disk or the network.
///
/// It exists so the web console can tell an administrator what is wrong *before* the file is saved: a broken
/// file would otherwise make the server panic at the next provider lookup (`get_providers_config_from_file`).
///
/// # Returns
/// `(errors, warnings)`. The file is usable only when `errors` is empty. An empty provider list is a warning
/// (login by password still works), not an error.
pub fn validate_config_toml(content: &str) -> (Vec<String>, Vec<String>) {
    let mut errors = Vec::new();
    let mut warnings = Vec::new();
    let config: Config = match toml::from_str(content) {
        Ok(c) => c,
        Err(e) => {
            // an empty or fully commented file is a valid TOML document that lacks the `provider` array
            if content.lines().all(|l| l.trim().is_empty() || l.trim_start().starts_with('#')) {
                warnings.push("No provider is configured: only the password login is available.".into());
            } else {
                errors.push(format!("Syntax error: {e}"));
            }
            return (errors, warnings);
        }
    };
    if config.provider.is_empty() {
        warnings.push("No provider is configured: only the password login is available.".into());
    }
    let mut seen_ops = std::collections::HashSet::new();
    for (i, p) in config.provider.iter().enumerate() {
        let label = format!("provider #{} ({})", i + 1, p.op);
        for (field, value) in [
            ("op", &p.op),
            ("app_id", &p.app_id),
            ("app_secret", &p.app_secret),
            ("scope", &p.scope),
            ("op_auth_string", &p.op_auth_string),
        ] {
            if value.trim().is_empty() {
                errors.push(format!("{label}: `{field}` must not be empty."));
            }
        }
        for (field, value) in [
            ("authorization_url", &p.authorization_url),
            ("token_exchange_url", &p.token_exchange_url),
        ] {
            if !is_http_url(value) {
                errors.push(format!("{label}: `{field}` must be an absolute http(s) URL."));
            }
        }
        if !p.op.trim().is_empty() && !seen_ops.insert(p.op.clone()) {
            errors.push(format!("{label}: `op` is used by another provider."));
        }
        // the clients send `op` and the server looks the provider up by `oidc/<op>`
        if !p.op.is_empty() && p.op_auth_string != format!("oidc/{}", p.op) {
            errors.push(format!("{label}: `op_auth_string` must be \"oidc/{}\".", p.op));
        }
        if p.app_id.starts_with("your_") || p.app_secret.starts_with("your_") {
            warnings.push(format!("{label}: `app_id` or `app_secret` still looks like a placeholder."));
        }
    }
    (errors, warnings)
}

#[cfg(test)]
mod tests {
    use super::*;

    const GOOD: &str = r#"
        [[provider]]
        provider = "Github"
        authorization_url = "https://github.com/login/oauth/authorize"
        token_exchange_url = "https://github.com/login/oauth/access_token"
        app_id = "id"
        app_secret = "secret"
        scope = "read:user"
        op_auth_string = "oidc/github"
        op = "github"
    "#;

    #[test]
    fn validate_accepts_a_good_file() {
        let (e, w) = validate_config_toml(GOOD);
        assert!(e.is_empty(), "{e:?}");
        assert!(w.is_empty(), "{w:?}");
    }

    #[test]
    fn validate_accepts_the_custom_alias_and_an_empty_file() {
        let (e, _) = validate_config_toml(&GOOD.replace("Github", "Custom"));
        assert!(e.is_empty(), "{e:?}");
        let (e, w) = validate_config_toml("# nothing\n");
        assert!(e.is_empty());
        assert_eq!(w.len(), 1);
    }

    #[test]
    fn validate_reports_the_problems() {
        let (e, _) = validate_config_toml("[[provider]\nx");
        assert!(e[0].starts_with("Syntax error"));
        let (e, _) = validate_config_toml(&GOOD.replace("Github", "Nope"));
        assert!(e[0].starts_with("Syntax error"), "unknown provider enum: {e:?}");
        let bad = GOOD
            .replace("https://github.com/login/oauth/authorize", "ftp://x")
            .replace("scope = \"read:user\"", "scope = \"\"")
            .replace("oidc/github", "oidc/other");
        let (e, _) = validate_config_toml(&bad);
        assert_eq!(e.len(), 3, "{e:?}");
        let two = format!("{GOOD}{GOOD}");
        let (e, _) = validate_config_toml(&two);
        assert!(e.iter().any(|m| m.contains("used by another")));
    }

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

        let providers = get_providers_config_from_file(config_file.path().to_str().unwrap());
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

        let providers = get_providers_config_from_file(config_file.path().to_str().unwrap());
        assert_eq!(providers.len(), 2);
    }

    #[test]
    fn test_decode_id_token() {
        let id_token = "eyJhbGciOiJSUzI1NiIsImtpZCI6IjhiMjFkMTM0NjExZDQxNWJkMWU2MjUzOGE0ZGRjOTA4NmYxYTZiMjUifQ.eyJpc3MiOiJodHRwczovL2RleC1tb2NrLXNlcnZlci5OT05FL2RleCIsInN1YiI6IkNpUXdPR0U0TmpnMFlpMWtZamc0TFRSaU56TXRPVEJoT1MwelkyUXhOall4WmpVME5qWVNCV3h2WTJGcyIsImF1ZCI6InNjdGdkZXNrLWFwaS1zZXJ2ZXIiLCJleHAiOjE3MTU2NzEwODQsImlhdCI6MTcxNTU4NDY4NCwiYXRfaGFzaCI6IjVvZEdyU3VrMW9lejJkc1NaRXZFM0EiLCJjX2hhc2giOiJfdFZfZFNiU09qTVVmRVdMeVVNSTNnIiwiZW1haWwiOiJhZG1pbkBkZXNrLk5PTkUiLCJlbWFpbF92ZXJpZmllZCI6dHJ1ZSwibmFtZSI6ImFkbWluIn0.AqOiwBKq2i_AoJcbfxuaVY54PN3GJjnHIn3E2FWoZY2IOu8qxvZevcUb4mjnoUZGf2QaabIcTAIxIg-mpFTRxheOPiQ1c9VSZ0vd-wNGrAG12vdraRq0-evqmFduR2G9k20QMIV8iHiGM7l93k8Fw5_bnTQId044BjepayS98bpUclS4RIIGoOLBM5IenfBCqLhHHv6oYUM6HDU4rCD02U9_Bu597wedeLdYYa7lzBDyb88ab83-eALsDpbFZ90rUnvAhpTQcl9_t51Etx-sP1yWSQ3UZ-QL61cKreqWlMbimM43R4boUWnpQTMF7ZO0EftVixEfaIQvWDRm-TLl8A";
        let (name, email) = crate::oauth_provider::decode_oauth_id_token(id_token).unwrap();
        assert_eq!(name, "admin");
    }
}
