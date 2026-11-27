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
    /// An empty or commented-out configuration is valid and means that password login is the only
    /// available login method.  The default is necessary because TOML files with no `[[provider]]`
    /// entries do not contain this field at all.
    #[serde(default)]
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
    if !std::path::Path::new(config_file).exists() {
        log::error!("oauth2 config file does not exist, creating it, we recommend you to fill it with your own values, you can change the file path by setting the OAUTH2_CONFIG_FILE environment variable.");
        let oauth2_config = include_str!("../../api-server/oauth2.toml");
        if let Err(error) = fs::write(config_file, oauth2_config) {
            log::error!(
                "Failed to create oauth2 config file '{}': {}",
                config_file,
                error
            );
            return Vec::new();
        }
    }
    let config_file_content = match fs::read_to_string(config_file) {
        Ok(content) => content,
        Err(error) => {
            log::error!(
                "Failed to read oauth2 config file '{}': {}",
                config_file,
                error
            );
            return Vec::new();
        }
    };

    match toml::from_str::<Config>(&config_file_content) {
        Ok(config) => config.provider,
        Err(error) => {
            log::error!(
                "Failed to parse oauth2 config file '{}': {}",
                config_file,
                error
            );
            Vec::new()
        }
    }
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
            if content
                .lines()
                .all(|l| l.trim().is_empty() || l.trim_start().starts_with('#'))
            {
                warnings.push(
                    "No provider is configured: only the password login is available.".into(),
                );
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
                errors.push(format!(
                    "{label}: `{field}` must be an absolute http(s) URL."
                ));
            }
        }
        if !p.op.trim().is_empty() && !seen_ops.insert(p.op.clone()) {
            errors.push(format!("{label}: `op` is used by another provider."));
        }
        // the clients send `op` and the server looks the provider up by `oidc/<op>`
        if !p.op.is_empty() && p.op_auth_string != format!("oidc/{}", p.op) {
            errors.push(format!(
                "{label}: `op_auth_string` must be \"oidc/{}\".",
                p.op
            ));
        }
        // `oidc_auth` of the API server has an implementation for these types only: the other names exist in the
        // enum but would make the login request panic, so a file that uses them must not be saved.
        if !matches!(
            p.provider,
            Provider::Github | Provider::Dex | Provider::Oauth2
        ) {
            errors.push(format!(
                "{label}: the provider type {:?} is not implemented by this server, use `Oauth2` (any standard OAuth2 / OpenID Connect provider), `Dex` or `Github`.",
                p.provider
            ));
        }
        if p.app_id.starts_with("your_") || p.app_secret.starts_with("your_") {
            warnings.push(format!(
                "{label}: `app_id` or `app_secret` still looks like a placeholder."
            ));
        }
    }
    (errors, warnings)
}

/// One line of the result of [`check_live`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LiveCheck {
    /// What was checked, for example `github: application`.
    pub target: String,
    /// `ok`, `warning` or `error`.
    pub level: &'static str,
    pub message: String,
}

/// How long the provider has to answer one request of [`check_live`].
const LIVE_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(10);

/// Reads the OAuth2 `error` of an answer of a token endpoint, which is JSON for most providers and a
/// url-encoded form for a few (GitHub when it is not asked for JSON).
fn token_error_code(body: &str) -> Option<String> {
    if let Ok(serde_json::Value::Object(map)) = serde_json::from_str::<serde_json::Value>(body) {
        return map
            .get("error")
            .and_then(|e| e.as_str())
            .map(str::to_string);
    }
    url::form_urlencoded::parse(body.as_bytes())
        .find(|(k, _)| k == "error")
        .map(|(_, v)| v.into_owned())
}

/// Decides what the answer of a token endpoint to a *dummy* authorization code says about the application.
///
/// The check sends `app_id`, `app_secret` and a code that cannot be valid. A provider that knows the
/// application rejects the code (`invalid_grant`, `bad_verification_code`...); one that does not know it
/// rejects the application first (`invalid_client`, GitHub's `incorrect_client_credentials`...). So the
/// *kind* of rejection tells whether the application is the right one, without anybody signing in.
///
/// # Returns
/// `(level, message)` with level `ok`, `warning` or `error`.
pub fn classify_token_answer(status: u16, body: &str) -> (&'static str, String) {
    // GitHub answers `{"error":"Not Found"}` with a 404 for an unknown `client_id`, so a 404 is a verdict
    // (wrong application or wrong address), not an inconclusive answer
    if status == 404 {
        return (
            "error",
            "Not found (HTTP 404): `app_id` is unknown to the provider, or `token_exchange_url` is wrong."
                .into(),
        );
    }
    match token_error_code(body).as_deref() {
        Some("invalid_client" | "incorrect_client_credentials" | "unauthorized_client") => (
            "error",
            "The provider does not recognise this application: check `app_id` and `app_secret`."
                .into(),
        ),
        Some("invalid_grant" | "bad_verification_code" | "redirect_uri_mismatch") => (
            "ok",
            "The provider accepts this application (it only rejected the test code).".into(),
        ),
        Some(other) => (
            "warning",
            format!("The provider answered `{other}` (HTTP {status}): this does not tell whether the application is right."),
        ),
        None if (200..300).contains(&status) => (
            "warning",
            format!("The token endpoint answered HTTP {status} without an error for a test code: is `token_exchange_url` the right address?"),
        ),
        None => (
            "error",
            format!("The token endpoint answered HTTP {status} and no OAuth2 error: is `token_exchange_url` the right address?"),
        ),
    }
}

/// Decides what the answer of an authorization endpoint says. A real page (2xx) or a redirection (to the login
/// page of the provider) means it knows the application; a 4xx means it refuses the request, most often because
/// of the `client_id` or the redirect URI.
pub fn classify_authorize_answer(status: u16) -> (&'static str, String) {
    match status {
        200..=399 => (
            "ok",
            format!("The authorization page answers (HTTP {status})."),
        ),
        400..=499 => (
            "error",
            format!("The provider refuses the sign-in request (HTTP {status}): check `app_id`, `authorization_url` and the redirect URI registered at the provider."),
        ),
        _ => (
            "error",
            format!("The authorization endpoint failed (HTTP {status})."),
        ),
    }
}

/// Checks each provider of an `oauth2.toml` against the real provider, without any user interaction:
/// the authorization endpoint must answer, and the token endpoint must recognise `app_id` / `app_secret`
/// (see [`classify_token_answer`]). A full sign-in cannot be done here, it needs a person at the provider's
/// login page: the console offers it as a separate test.
///
/// # Parameters
/// * `content` - the TOML text (not necessarily saved).
/// * `callback_url` - the redirect URI the server really sends to the providers.
///
/// # Returns
/// One entry per provider and per endpoint. A text that does not parse gives a single `error` entry.
pub async fn check_live(content: &str, callback_url: &str) -> Vec<LiveCheck> {
    let config: Config = match toml::from_str(content) {
        Ok(c) => c,
        Err(e) => {
            return vec![LiveCheck {
                target: "file".into(),
                level: "error",
                message: format!("Syntax error: {e}"),
            }]
        }
    };
    // never follow a redirection: the answer of the first hop is what is judged, and a provider cannot send
    // the server somewhere else
    let client = match reqwest::Client::builder()
        .timeout(LIVE_TIMEOUT)
        .redirect(reqwest::redirect::Policy::none())
        .build()
    {
        Ok(c) => c,
        Err(e) => {
            return vec![LiveCheck {
                target: "http".into(),
                level: "error",
                message: e.to_string(),
            }]
        }
    };
    let mut results = Vec::new();
    for p in &config.provider {
        let target = |what: &str| format!("{}: {what}", p.op);
        let authorize = client
            .get(&p.authorization_url)
            .query(&[
                ("client_id", p.app_id.as_str()),
                ("redirect_uri", callback_url),
                ("response_type", "code"),
                ("scope", p.scope.as_str()),
                ("state", "sctgdesk-config-check"),
            ])
            .send()
            .await;
        results.push(match authorize {
            Ok(r) => {
                let (level, message) = classify_authorize_answer(r.status().as_u16());
                LiveCheck {
                    target: target("authorization endpoint"),
                    level,
                    message,
                }
            }
            Err(e) => LiveCheck {
                target: target("authorization endpoint"),
                level: "error",
                message: format!("Cannot reach `authorization_url`: {}", short_error(e)),
            },
        });
        let token = client
            .post(&p.token_exchange_url)
            .header("Accept", "application/json")
            .form(&[
                ("grant_type", "authorization_code"),
                ("client_id", p.app_id.as_str()),
                ("client_secret", p.app_secret.as_str()),
                ("code", "sctgdesk-config-check"),
                ("redirect_uri", callback_url),
            ])
            .send()
            .await;
        results.push(match token {
            Ok(r) => {
                let status = r.status().as_u16();
                let body = r.text().await.unwrap_or_default();
                let (level, message) = classify_token_answer(status, &body);
                LiveCheck {
                    target: target("application"),
                    level,
                    message,
                }
            }
            Err(e) => LiveCheck {
                target: target("application"),
                level: "error",
                message: format!("Cannot reach `token_exchange_url`: {}", short_error(e)),
            },
        });
    }
    if results.is_empty() {
        results.push(LiveCheck {
            target: "file".into(),
            level: "warning",
            message: "No provider is configured: nothing to check.".into(),
        });
    }
    results
}

/// The part of a `reqwest` error worth showing: its text without the URL (which may hold the query).
fn short_error(e: reqwest::Error) -> String {
    if e.is_timeout() {
        "timeout".into()
    } else if e.is_connect() {
        "connection refused or host not found".into()
    } else {
        e.without_url().to_string()
    }
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
    fn empty_oauth2_config_returns_no_providers_without_panicking() {
        for content in [
            "",
            "\n",
            "# No oauth2 provider configured.\n",
            " # No oauth2 provider configured.\n\n",
            "provider = []\n",
        ] {
            let config_file = tempfile::NamedTempFile::new().unwrap();
            std::fs::write(config_file.path(), content).unwrap();

            let providers = get_providers_config_from_file(config_file.path().to_str().unwrap());
            assert!(
                providers.is_empty(),
                "configuration {content:?} should disable all OAuth2 providers"
            );
        }
    }

    #[test]
    fn malformed_oauth2_config_returns_no_providers_without_panicking() {
        let config_file = tempfile::NamedTempFile::new().unwrap();
        std::fs::write(config_file.path(), "[[provider]\n").unwrap();

        let providers = get_providers_config_from_file(config_file.path().to_str().unwrap());
        assert!(providers.is_empty());
    }

    #[test]
    fn missing_oauth2_config_file_is_created_from_the_template() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("oauth2.toml");
        assert!(!path.exists());

        let providers = get_providers_config_from_file(path.to_str().unwrap());
        assert!(path.exists(), "the default file must be written");
        assert!(
            !providers.is_empty(),
            "the default file ships a sample provider"
        );
    }

    #[test]
    fn unwritable_location_returns_no_providers_without_panicking() {
        let dir = tempfile::tempdir().unwrap();
        // The parent directory does not exist: creating the default file fails.
        let path = dir.path().join("missing-dir").join("oauth2.toml");

        let providers = get_providers_config_from_file(path.to_str().unwrap());
        assert!(providers.is_empty());
    }

    #[test]
    fn unreadable_oauth2_config_returns_no_providers_without_panicking() {
        let dir = tempfile::tempdir().unwrap();
        // A directory exists but cannot be read as a file.
        let providers = get_providers_config_from_file(dir.path().to_str().unwrap());
        assert!(providers.is_empty());
    }

    #[test]
    fn non_utf8_oauth2_config_returns_no_providers_without_panicking() {
        let config_file = tempfile::NamedTempFile::new().unwrap();
        std::fs::write(config_file.path(), [0xff, 0xfe, 0x00, 0x80]).unwrap();

        let providers = get_providers_config_from_file(config_file.path().to_str().unwrap());
        assert!(providers.is_empty());
    }

    #[test]
    fn provider_of_the_wrong_type_returns_no_providers_without_panicking() {
        let config_file = tempfile::NamedTempFile::new().unwrap();
        std::fs::write(config_file.path(), "provider = \"not a list\"\n").unwrap();

        let providers = get_providers_config_from_file(config_file.path().to_str().unwrap());
        assert!(providers.is_empty());
    }

    #[test]
    fn configured_providers_are_still_returned() {
        let config_file = tempfile::NamedTempFile::new().unwrap();
        std::fs::write(
            config_file.path(),
            "# a comment\n[[provider]]\nprovider = \"Github\"\nauthorization_url = \"https://github.com/login/oauth/authorize\"\ntoken_exchange_url = \"https://github.com/login/oauth/access_token\"\napp_id = \"id\"\napp_secret = \"secret\"\nscope = \"read:user\"\nop_auth_string = \"oidc/github\"\nop = \"github\"\n",
        )
        .unwrap();

        let providers = get_providers_config_from_file(config_file.path().to_str().unwrap());
        assert_eq!(providers.len(), 1);
        assert_eq!(providers[0].op_auth_string, "oidc/github");
    }

    #[test]
    fn config_without_provider_field_deserializes_to_an_empty_list() {
        let config: Config = toml::from_str("# No oauth2 provider configured.\n").unwrap();
        assert!(config.provider.is_empty());
    }

    #[test]
    fn validate_reports_the_problems() {
        let (e, _) = validate_config_toml("[[provider]\nx");
        assert!(e[0].starts_with("Syntax error"));
        let (e, _) = validate_config_toml(&GOOD.replace("Github", "Nope"));
        assert!(
            e[0].starts_with("Syntax error"),
            "unknown provider enum: {e:?}"
        );
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

    #[test]
    fn token_answers_tell_the_application_apart() {
        // GitHub (JSON when asked, url-encoded otherwise), Google and a standard server
        assert_eq!(
            classify_token_answer(200, r#"{"error":"incorrect_client_credentials"}"#).0,
            "error"
        );
        assert_eq!(
            classify_token_answer(200, "error=incorrect_client_credentials&x=1").0,
            "error"
        );
        assert_eq!(
            classify_token_answer(401, r#"{"error":"invalid_client"}"#).0,
            "error"
        );
        assert_eq!(
            classify_token_answer(200, r#"{"error":"bad_verification_code"}"#).0,
            "ok"
        );
        assert_eq!(
            classify_token_answer(400, r#"{"error":"invalid_grant"}"#).0,
            "ok"
        );
        assert_eq!(
            classify_token_answer(400, r#"{"error":"invalid_request"}"#).0,
            "warning"
        );
        assert_eq!(classify_token_answer(404, "<html>").0, "error");
        assert_eq!(
            classify_token_answer(404, r#"{"error":"Not Found"}"#).0,
            "error"
        );
        assert_eq!(classify_authorize_answer(302).0, "ok");
        assert_eq!(classify_authorize_answer(404).0, "error");
    }

    #[test]
    fn unimplemented_provider_types_are_refused() {
        let (e, _) = validate_config_toml(&GOOD.replace("Github", "Google"));
        assert!(e.iter().any(|m| m.contains("not implemented")), "{e:?}");
    }
}
