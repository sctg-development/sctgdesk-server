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
use std::{future::Future, pin::Pin};

use crate::{
    errors::Oauth2Error,
    oauth_provider::{decode_oauth2_id_token, OAuthProvider, OAuthProviderFactory, OAuthResponse},
    Provider, ProviderConfig, TokenResponse,
};
use base64::prelude::{Engine as _, BASE64_STANDARD};

use url::form_urlencoded;

pub struct Oauth2Provider {
    provider_config: ProviderConfig,
}

/// Get the authorization header for the provider
///
/// # Arguments
/// * `provider_config` - The provider configuration
///
/// # Returns
/// The authorization header
fn get_authorization_header(provider_config: &ProviderConfig) -> String {
    format!(
        "Basic {}",
        BASE64_STANDARD.encode(format!(
            "{}:{}",
            provider_config.app_id, provider_config.app_secret
        ))
    )
}

impl OAuthProviderFactory for Oauth2Provider {
    fn new() -> Self {
        let provider_config = Self::get_provider_config(Provider::Oauth2);
        Self { provider_config }
    }
}
impl OAuthProvider for Oauth2Provider {
    fn get_redirect_url(&self, callback_url: &str, state: &str) -> String {
        let redirect_url =
            form_urlencoded::byte_serialize(callback_url.as_bytes()).collect::<String>();
        let scope = form_urlencoded::byte_serialize(self.provider_config.scope.as_bytes())
            .collect::<String>();
        let state = form_urlencoded::byte_serialize(state.as_bytes()).collect::<String>();

        format!(
            "{}?client_id={}&redirect_uri={}&response_type=code&scope={}&state={}",
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
            let authorization_header = get_authorization_header(&provider_config);
            let response = reqwest::Client::new()
                .post(provider_config.token_exchange_url.as_str())
                .header("Authorization", authorization_header)
                .header("Content-Type", "application/x-www-form-urlencoded")
                .form(&[
                    ("grant_type", "authorization_code"),
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

            let body = response
                .json::<TokenResponse>()
                .await
                .map_err(|_| Oauth2Error::ExchangeCodeError)?;

            if let Some(id_token) = body.id_token {
                let (username, email) = decode_oauth2_id_token(&id_token)?;
                Ok(OAuthResponse {
                    access_token: body.access_token,
                    username,
                    email,
                })
            } else {
                Err(Oauth2Error::ExchangeCodeError)
            }
        })
    }

    fn get_provider_type(&self) -> crate::Provider {
        Provider::Oauth2
    }
}
