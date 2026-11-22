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
// The AWS SDK is optional (feature `aws`, on by default): it only signs the download links of the clients, it weighs
// about 2 MB of code, and the builds for small routers leave it out. Without it the configuration can still be read,
// edited and validated, and the functions that talk to the storage answer that S3 is not built in.
#![cfg_attr(not(feature = "aws"), allow(dead_code, unused_imports))]
#[cfg(feature = "aws")]
use aws_config::{BehaviorVersion, ConfigLoader};
#[cfg(feature = "aws")]
use aws_sdk_s3::config::SharedHttpClient;
#[cfg(feature = "aws")]
use aws_sdk_s3::presigning::PresigningConfig;
#[cfg(feature = "aws")]
use aws_sdk_s3::{
    config::{Credentials, Region},
    Client,
};
#[cfg(feature = "aws")]
use aws_smithy_http_client::{tls, Builder as HttpClientBuilder};
use regex::Regex;
use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::sync::{Mutex, OnceLock};
use std::time::{Duration, Instant};

#[derive(Deserialize, Debug, Clone)]
pub struct S3Config {
    #[serde(rename = "Page")]
    pub page: String,
    #[serde(rename = "Endpoint")]
    pub endpoint: String,
    #[serde(rename = "Region")]
    pub region: String,
    #[serde(rename = "AccessKey")]
    pub access_key: String,
    #[serde(rename = "SecretKey")]
    pub secret_key: String,
    #[serde(rename = "Bucket")]
    pub bucket: String,
    #[serde(rename = "Windows64Key")]
    pub windows64_key: String,
    #[serde(rename = "Windows32Key")]
    pub windows32_key: String,
    #[serde(rename = "OSXKey")]
    pub osxkey: String,
    #[serde(rename = "OSXArm64Key")]
    pub osxarm64_key: String,
    #[serde(rename = "IOSKey")]
    pub ioskey: String,
    /// Only to find the newest version of a key containing `{version}` (see `resolve_key`).
    /// Set it to the real bucket when `Bucket` is an Oracle Cloud namespace and the keys start with
    /// the bucket (`master/...`): the layer of Oracle that speaks S3 wants the namespace in the host
    /// name and the bucket in the path.
    #[serde(rename = "ListBucket", default)]
    pub list_bucket: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub s3config: S3Config,
}
#[cfg(feature = "aws")]
/// Get object using presigned request.
async fn get_object(
    client: &Client,
    bucket: &str,
    object: &str,
    expires_in: u64,
) -> Result<String, Box<dyn Error>> {
    let expires_in = Duration::from_secs(expires_in);
    let presigned_request = client
        .get_object()
        .bucket(bucket)
        .key(object)
        .presigned(PresigningConfig::expires_in(expires_in)?)
        .await?;

    Ok(presigned_request.uri().to_string())
}

#[cfg(feature = "aws")]
pub async fn get_signed_release_url(
    endpoint: &str,
    region: &str,
    bucket: &str,
    object: &str,
    expires_in: u64,
) -> Result<String, Box<dyn Error>> {
    let region = region.to_string();
    let region_provider = Region::new(region);

    let shared_config = ConfigLoader::default()
        .behavior_version(BehaviorVersion::latest())
        .http_client(https_client())
        .region(region_provider)
        .endpoint_url(endpoint)
        .load()
        .await;
    let client = Client::new(&shared_config);
    get_object(&client, &bucket, &object, expires_in).await
}

/// Template shown by the console when the S3 config file does not exist yet.
pub const CONFIG_TEMPLATE: &str = include_str!("../../api-server/s3config.toml");

/// Validates the text of an `s3config.toml` without touching the disk or the network.
///
/// # Returns
/// `(errors, warnings)`. Errors make the file unusable (syntax, missing key, bad endpoint URL); warnings are
/// things that parse but will not work (empty or placeholder credentials).
pub fn validate_config_toml(content: &str) -> (Vec<String>, Vec<String>) {
    let mut errors = Vec::new();
    let mut warnings = Vec::new();
    let config: Config = match toml::from_str(content) {
        Ok(c) => c,
        Err(e) => {
            errors.push(format!("Syntax error: {e}"));
            return (errors, warnings);
        }
    };
    let s = &config.s3config;
    let is_http = |v: &str| {
        url::Url::parse(v)
            .map(|u| matches!(u.scheme(), "http" | "https") && u.host_str().is_some())
            .unwrap_or(false)
    };
    if !is_http(&s.endpoint) {
        errors.push("`Endpoint` must be an absolute http(s) URL.".into());
    }
    // `Page` is only used to find the latest version number: it may stay empty if the keys are fixed
    if !s.page.is_empty() && !is_http(&s.page) {
        errors.push("`Page` must be an absolute http(s) URL.".into());
    }
    for (name, value) in [
        ("Region", &s.region),
        ("Bucket", &s.bucket),
        ("Windows64Key", &s.windows64_key),
        ("Windows32Key", &s.windows32_key),
        ("OSXKey", &s.osxkey),
        ("OSXArm64Key", &s.osxarm64_key),
        ("IOSKey", &s.ioskey),
    ] {
        if value.trim().is_empty() {
            errors.push(format!("`{name}` must not be empty."));
        }
    }
    for (name, value) in [("AccessKey", &s.access_key), ("SecretKey", &s.secret_key)] {
        if value.trim().is_empty() {
            warnings.push(format!(
                "`{name}` is empty: the download links cannot be signed."
            ));
        } else if value.starts_with("your_") {
            warnings.push(format!("`{name}` still looks like a placeholder."));
        }
    }
    (errors, warnings)
}

/// One line of the result of [`check_live`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LiveCheck {
    /// What was checked, for example `OSXKey`.
    pub target: String,
    /// `ok`, `warning` or `error`.
    pub level: &'static str,
    pub message: String,
}

#[cfg(feature = "aws")]
/// How long the bucket has to answer one request of [`check_live`].
const LIVE_TIMEOUT: Duration = Duration::from_secs(15);

#[cfg(feature = "aws")]
/// The five download keys of `s3config.toml`, with the name shown to the administrator.
fn download_keys(s: &S3Config) -> [(&'static str, &str); 5] {
    [
        ("Windows64Key", &s.windows64_key),
        ("Windows32Key", &s.windows32_key),
        ("OSXKey", &s.osxkey),
        ("OSXArm64Key", &s.osxarm64_key),
        ("IOSKey", &s.ioskey),
    ]
}

#[cfg(feature = "aws")]
/// Explains a failed request to the bucket. A `ServiceError` means the server was reached and answered (so the
/// connection works) with an HTTP status that tells what is wrong; anything else is a network failure.
///
/// # Returns
/// `(reached, message)`: whether the S3 endpoint answered at all, and what to show.
fn explain_s3_error<E: std::fmt::Debug, R>(
    error: &aws_sdk_s3::error::SdkError<E, R>,
    status: impl Fn(&R) -> u16,
) -> (bool, String) {
    use aws_sdk_s3::error::SdkError;
    match error {
        SdkError::ServiceError(e) => {
            let code = status(e.raw());
            let text = match code {
                404 => "the file does not exist in the bucket (check the key, and `Bucket`)"
                    .to_string(),
                403 => {
                    "access denied: check `AccessKey` / `SecretKey` and their rights on the bucket"
                        .to_string()
                }
                301 | 307 => {
                    "the bucket is in another region or endpoint: check `Region` and `Endpoint`"
                        .to_string()
                }
                other => format!("the storage answered HTTP {other}"),
            };
            (true, text)
        }
        SdkError::TimeoutError(_) => (false, "timeout: the endpoint did not answer".into()),
        SdkError::DispatchFailure(_) => (
            false,
            "cannot connect to the endpoint (check `Endpoint`, DNS and the network of the server)"
                .into(),
        ),
        other => (false, format!("{other}")),
    }
}

#[cfg(feature = "aws")]
/// Checks an `s3config.toml` against the real storage, with the credentials it holds: for each download key,
/// asks the bucket for the first byte of the object (the same signed `GET` that a user's download does, so a
/// success here means the download works). A key with `{version}` is first resolved to the newest object.
///
/// The first request also tells whether the connection works at all: when the endpoint cannot be reached the
/// check stops there instead of waiting for a timeout per file.
///
/// # Returns
/// A `connection` line, then one line per key. A text that does not parse gives a single `error` line.
pub async fn check_live(content: &str) -> Vec<LiveCheck> {
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
    let s = &config.s3config;
    let credentials = Credentials::new(&s.access_key, &s.secret_key, None, None, "s3config.toml");
    let shared = ConfigLoader::default()
        .behavior_version(BehaviorVersion::latest())
        .http_client(https_client())
        .region(Region::new(s.region.clone()))
        .endpoint_url(&s.endpoint)
        .credentials_provider(credentials)
        .load()
        .await;
    let client = Client::new(&shared);

    let mut lines = Vec::new();
    let mut connection: Option<LiveCheck> = None;
    for (name, template) in download_keys(s) {
        let resolved =
            match tokio::time::timeout(LIVE_TIMEOUT, resolve_key(&config, template)).await {
                Ok(Ok(r)) => r,
                Ok(Err(e)) => {
                    lines.push(LiveCheck {
                        target: name.into(),
                        level: "error",
                        message: format!("cannot find the newest version of `{template}`: {e}"),
                    });
                    continue;
                }
                Err(_) => {
                    lines.push(LiveCheck {
                        target: name.into(),
                        level: "error",
                        message: "timeout while listing the bucket".into(),
                    });
                    continue;
                }
            };
        let request = client
            .get_object()
            .bucket(&s.bucket)
            .key(&resolved.key)
            .range("bytes=0-0")
            .send();
        let line = match tokio::time::timeout(LIVE_TIMEOUT, request).await {
            Ok(Ok(_)) => {
                connection.get_or_insert(LiveCheck {
                    target: "connection".into(),
                    level: "ok",
                    message: "The storage answers and accepts the credentials.".into(),
                });
                LiveCheck {
                    target: name.into(),
                    level: "ok",
                    message: format!("`{}` is downloadable.", resolved.key),
                }
            }
            Ok(Err(e)) => {
                let (reached, why) = explain_s3_error(&e, |r| r.status().as_u16());
                if reached {
                    // answered: the connection is fine even when this file is not
                    connection.get_or_insert(LiveCheck {
                        target: "connection".into(),
                        level: if why.contains("access denied") {
                            "error"
                        } else {
                            "ok"
                        },
                        message: if why.contains("access denied") {
                            why.clone()
                        } else {
                            "The storage answers.".into()
                        },
                    });
                } else {
                    // nothing reached: say it once and stop, the other keys would fail the same way
                    lines.insert(
                        0,
                        LiveCheck {
                            target: "connection".into(),
                            level: "error",
                            message: why,
                        },
                    );
                    return lines;
                }
                LiveCheck {
                    target: name.into(),
                    level: "error",
                    message: format!("`{}`: {why}.", resolved.key),
                }
            }
            Err(_) => {
                lines.insert(
                    0,
                    LiveCheck {
                        target: "connection".into(),
                        level: "error",
                        message: "timeout: the endpoint did not answer".into(),
                    },
                );
                return lines;
            }
        };
        lines.push(line);
    }
    if let Some(c) = connection {
        lines.insert(0, c);
    }
    lines
}

pub async fn parse_config(config: &str) -> Result<Config, Box<dyn Error>> {
    let config = toml::from_str(config);
    if config.is_err() {
        return Err("Error parsing config".into());
    }
    Ok(config.unwrap())
}

#[cfg(feature = "aws")]
/// Get the name of the s3 config file
/// from the S3COONFIG_FILE environment variable or
/// default to "s3config.toml"
///
/// # Returns
/// The s3config
/// The HTTPS client of the SDK, on rustls with the ring provider (see Cargo.toml).
fn https_client() -> SharedHttpClient {
    HttpClientBuilder::new()
        .tls_provider(tls::Provider::Rustls(
            tls::rustls_provider::CryptoMode::Ring,
        ))
        .build_https()
}

pub async fn get_s3_config_file() -> Result<Config, Box<dyn Error>> {
    let config_filename =
        std::env::var("S3CONFIG_FILE").unwrap_or_else(|_| "s3config.toml".to_string());
    // If file does not exist create it
    if !std::path::Path::new(&config_filename).exists() {
        log::error!("S3 config file does not exist, creating it, we recommend you to fill it with your own values, you can change the file path by setting the S3CONFIG_FILE environment variable.");
        let s3_config = include_str!("../../api-server/s3config.toml");
        fs::write(&config_filename, s3_config).expect("Failed to write s3 config file");
    }
    let config_file_content =
        fs::read_to_string(config_filename).expect("Failed to read s3 config file");
    parse_config(&config_file_content).await
}

#[cfg(feature = "aws")]
pub async fn get_signed_release_url_with_config(
    config: Config,
    key: &str,
) -> Result<String, Box<dyn Error>> {
    std::env::set_var("AWS_ACCESS_KEY_ID", config.s3config.access_key);
    std::env::set_var("AWS_SECRET_ACCESS_KEY", config.s3config.secret_key);
    get_signed_release_url(
        &config.s3config.endpoint.as_str(),
        config.s3config.region.as_str(),
        config.s3config.bucket.as_str(),
        key,
        900,
    )
    .await
}

/// What the functions that need the AWS SDK answer when the crate is built without the `aws` feature.
#[cfg(not(feature = "aws"))]
const NOT_BUILT: &str =
    "S3 support is not built into this binary (built without the `aws` feature)";

/// Without the `aws` feature: always an error, see [`NOT_BUILT`].
#[cfg(not(feature = "aws"))]
pub async fn get_latest_signed_release_url(
    _config: Config,
    _template: &str,
) -> Result<String, BoxError> {
    Err(NOT_BUILT.into())
}

/// Without the `aws` feature: always an error, see [`NOT_BUILT`].
#[cfg(not(feature = "aws"))]
pub async fn extract_version() -> Result<String, Box<dyn std::error::Error>> {
    Err(NOT_BUILT.into())
}

/// Without the `aws` feature: a single `error` line saying so.
#[cfg(not(feature = "aws"))]
pub async fn check_live(_content: &str) -> Vec<LiveCheck> {
    vec![LiveCheck {
        target: "s3".into(),
        level: "error",
        message: NOT_BUILT.into(),
    }]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_the_shipped_template_and_bad_files() {
        let (e, _) = validate_config_toml(CONFIG_TEMPLATE);
        assert!(e.is_empty(), "{e:?}");
        let (e, _) = validate_config_toml("[s3config]\nPage = \"x\"");
        assert!(e[0].starts_with("Syntax error"));
        let bad = CONFIG_TEMPLATE.replace(
            "https://compat.objectstorage",
            "nope://compat.objectstorage",
        );
        let (e, _) = validate_config_toml(&bad);
        assert_eq!(e.len(), 1, "{e:?}");
        let empty_key = CONFIG_TEMPLATE.replace(
            "AccessKey = \"c324ead11faa0d87337c07ddc4a1129fab76188d\"",
            "AccessKey = \"\"",
        );
        let (e, w) = validate_config_toml(&empty_key);
        assert!(e.is_empty());
        assert_eq!(w.len(), 1);
    }
    #[cfg(feature = "aws")]
    use reqwest::StatusCode;

    #[cfg(feature = "aws")]
    /// With an endpoint nobody listens on, the check reports the connection (once) instead of one error per file.
    #[tokio::test]
    async fn live_check_reports_an_unreachable_endpoint() {
        let line = CONFIG_TEMPLATE
            .lines()
            .find(|l| l.starts_with("Endpoint"))
            .unwrap();
        let content = CONFIG_TEMPLATE.replace(line, "Endpoint = \"https://127.0.0.1:1\"");
        let lines = check_live(&content).await;
        assert_eq!(lines[0].target, "connection", "{lines:?}");
        assert_eq!(lines[0].level, "error");
        let bad = check_live("nope").await;
        assert_eq!(bad[0].level, "error");
    }

    #[cfg(feature = "aws")]
    /// The HTTPS client must work with the one rustls provider of the build (ring): a second
    /// provider makes rustls panic when it is first used. A closed local port is enough to get there.
    #[tokio::test]
    async fn the_https_client_reaches_the_tls_layer() {
        let shared_config = ConfigLoader::default()
            .behavior_version(BehaviorVersion::latest())
            .http_client(https_client())
            .region(Region::new("us-east-1"))
            .endpoint_url("https://127.0.0.1:1")
            .credentials_provider(aws_sdk_s3::config::Credentials::new(
                "k", "s", None, None, "test",
            ))
            .load()
            .await;
        let client = Client::new(&shared_config);
        let result = client.head_bucket().bucket("b").send().await;
        assert!(result.is_err(), "nothing listens on this port");
    }

    #[cfg(feature = "aws")]
    #[tokio::test]
    #[ignore = "calls the live S3 bucket described by s3config.toml; run explicitly with --ignored"]
    async fn test_get_signed_release_url() {
        let config = get_s3_config_file().await;
        if config.is_err() {
            println!("Error parsing config");
        }
        let config: Config = config.unwrap();

        std::env::set_var("AWS_ACCESS_KEY_ID", config.s3config.access_key);
        std::env::set_var("AWS_SECRET_ACCESS_KEY", config.s3config.secret_key);
        let url = get_signed_release_url(
            &config.s3config.endpoint.as_str(),
            config.s3config.region.as_str(),
            config.s3config.bucket.as_str(),
            config.s3config.osxkey.as_str(),
            900,
        )
        .await
        .unwrap();
        println!("Signed URL: {}", url);
        let response = reqwest::Client::new()
            .get(&url)
            .send()
            .await
            .expect("Failed to execute request.");
        // Save the status before dropping the response body
        let status = response.status();

        // Immediately drop the response body to avoid downloading the file
        drop(response.text().await);

        assert_eq!(status, StatusCode::OK);
    }
}

type BoxError = Box<dyn Error + Send + Sync>;

/// A key of `s3config.toml` that contains this stands for the newest version in the bucket:
/// `master/sctgdesk-releases/sctgdesk-{version}-x86_64.exe`.
const VERSION_PLACEHOLDER: &str = "{version}";
/// How long the newest key found in the bucket is reused before the bucket is listed again.
const LATEST_CACHE_TTL: Duration = Duration::from_secs(300);

/// The object designated by a key of `s3config.toml`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedKey {
    pub key: String,
    /// The `x.y.z` of the key, when it has one.
    pub version: Option<String>,
}

/// `1.5.0` is `[1, 5, 0]`; anything else (`1.5`, `1.5.0-rc1`, `web`) is not a version.
fn parse_version(text: &str) -> Option<[u64; 3]> {
    let mut numbers = text.split('.').map(|part| {
        if !part.is_empty() && part.bytes().all(|b| b.is_ascii_digit()) {
            part.parse::<u64>().ok()
        } else {
            None
        }
    });
    let version = [numbers.next()??, numbers.next()??, numbers.next()??];
    numbers.next().is_none().then_some(version)
}

/// Among `keys`, the one that matches `template` with the highest version in place of `{version}`.
/// The whole key has to match: `sctgdesk-{version}.dmg` does not pick `sctgdesk-1.4.0-web-basic.zip`.
pub fn pick_latest<'a>(
    template: &str,
    keys: impl IntoIterator<Item = &'a str>,
) -> Option<ResolvedKey> {
    let (head, tail) = template.split_once(VERSION_PLACEHOLDER)?;
    keys.into_iter()
        .filter_map(|key| {
            let version = key.strip_prefix(head)?.strip_suffix(tail)?;
            Some((parse_version(version)?, key, version))
        })
        .max_by_key(|(number, _, _)| *number)
        .map(|(_, key, version)| ResolvedKey {
            key: key.to_string(),
            version: Some(version.to_string()),
        })
}

/// Where and how to list the keys that start with a prefix.
#[derive(Debug, PartialEq, Eq)]
struct ListingTarget {
    endpoint: String,
    bucket: String,
    path_style: bool,
    /// What the keys of `s3config.toml` have in front of the object name inside the bucket.
    lead: String,
    /// The prefix to ask the bucket for.
    prefix: String,
}

fn listing_target(config: &S3Config, prefix: &str) -> Result<ListingTarget, BoxError> {
    match config.list_bucket.as_deref() {
        None => Ok(ListingTarget {
            endpoint: config.endpoint.clone(),
            bucket: config.bucket.clone(),
            path_style: false,
            lead: String::new(),
            prefix: prefix.to_string(),
        }),
        Some(bucket) => {
            let (scheme, host) = config
                .endpoint
                .split_once("://")
                .ok_or("Endpoint has no scheme")?;
            let lead = format!("{bucket}/");
            let prefix = prefix
                .strip_prefix(&lead)
                .ok_or_else(|| {
                    format!("the key {prefix:?} does not start with {lead:?} (ListBucket)")
                })?
                .to_string();
            Ok(ListingTarget {
                endpoint: format!("{scheme}://{}.{host}", config.bucket),
                bucket: bucket.to_string(),
                path_style: true,
                lead,
                prefix,
            })
        }
    }
}

#[cfg(feature = "aws")]
/// All the keys of the bucket that start with `prefix`, in the form used by `s3config.toml`.
async fn list_keys(config: &S3Config, prefix: &str) -> Result<Vec<String>, BoxError> {
    let target = listing_target(config, prefix)?;
    let credentials = Credentials::new(
        &config.access_key,
        &config.secret_key,
        None,
        None,
        "s3config.toml",
    );
    let shared_config = ConfigLoader::default()
        .behavior_version(BehaviorVersion::latest())
        .http_client(https_client())
        .region(Region::new(config.region.clone()))
        .endpoint_url(&target.endpoint)
        .credentials_provider(credentials)
        .load()
        .await;
    let s3_config = aws_sdk_s3::config::Builder::from(&shared_config)
        .force_path_style(target.path_style)
        .build();
    let client = Client::from_conf(s3_config);
    let mut pages = client
        .list_objects_v2()
        .bucket(&target.bucket)
        .prefix(&target.prefix)
        .into_paginator()
        .send();
    let mut keys = Vec::new();
    while let Some(page) = pages.next().await {
        for object in page?.contents() {
            if let Some(key) = object.key() {
                keys.push(format!("{}{key}", target.lead));
            }
        }
    }
    Ok(keys)
}

#[cfg(feature = "aws")]
fn latest_cache() -> &'static Mutex<HashMap<String, (Instant, ResolvedKey)>> {
    static CACHE: OnceLock<Mutex<HashMap<String, (Instant, ResolvedKey)>>> = OnceLock::new();
    CACHE.get_or_init(|| Mutex::new(HashMap::new()))
}

#[cfg(feature = "aws")]
/// The object designated by a key of `s3config.toml`.
///
/// A key without `{version}` is the object itself, nothing is asked to the bucket. A key with
/// `{version}` designates the newest version found in the bucket (the bucket is listed at most once
/// every few minutes).
pub async fn resolve_key(config: &Config, template: &str) -> Result<ResolvedKey, BoxError> {
    let Some((head, _)) = template.split_once(VERSION_PLACEHOLDER) else {
        let version = Regex::new(r"\d+\.\d+\.\d+")?
            .find(template)
            .map(|found| found.as_str().to_string());
        return Ok(ResolvedKey {
            key: template.to_string(),
            version,
        });
    };
    {
        let cache = latest_cache().lock().unwrap_or_else(|e| e.into_inner());
        if let Some((at, hit)) = cache.get(template) {
            if at.elapsed() < LATEST_CACHE_TTL {
                return Ok(hit.clone());
            }
        }
    }
    let keys = list_keys(&config.s3config, head).await?;
    let latest = pick_latest(template, keys.iter().map(String::as_str))
        .ok_or_else(|| format!("no object in the bucket matches {template:?}"))?;
    latest_cache()
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .insert(template.to_string(), (Instant::now(), latest.clone()));
    Ok(latest)
}

#[cfg(feature = "aws")]
/// Presigned URL of the object designated by a key of `s3config.toml` (see `resolve_key`).
pub async fn get_latest_signed_release_url(
    config: Config,
    template: &str,
) -> Result<String, BoxError> {
    let resolved = resolve_key(&config, template).await?;
    get_signed_release_url_with_config(config, &resolved.key)
        .await
        .map_err(|e| e.to_string().into())
}

#[cfg(feature = "aws")]
/// The version of the first platform key that has one: the newest one for a key with `{version}`.
pub async fn extract_version() -> Result<String, Box<dyn std::error::Error>> {
    let config = get_s3_config_file().await?;

    let keys = [
        &config.s3config.windows64_key,
        &config.s3config.windows32_key,
        &config.s3config.osxkey,
        &config.s3config.osxarm64_key,
        &config.s3config.ioskey,
    ];

    for key in &keys {
        match resolve_key(&config, key).await {
            Ok(ResolvedKey {
                version: Some(version),
                ..
            }) => return Ok(version),
            Ok(_) => {}
            Err(err) => log::warn!("cannot resolve {key:?}: {err}"),
        }
    }

    Err("No version found".into())
}

pub async fn get_software_download_page() -> Result<String, Box<dyn std::error::Error>> {
    let config = get_s3_config_file().await?;
    Ok(config.s3config.page)
}

#[cfg(test)]
mod latest_tests {
    use super::*;

    const KEYS: &[&str] = &[
        "master/sctgdesk-releases/keys/id_ed25519",
        "master/sctgdesk-releases/sctgdesk-1.2.5-x86_64.exe",
        "master/sctgdesk-releases/sctgdesk-1.2.6-x86_64.exe",
        "master/sctgdesk-releases/sctgdesk-1.2.6-i686.exe",
        "master/sctgdesk-releases/sctgdesk-1.10.0-x86_64.exe",
        "master/sctgdesk-releases/sctgdesk-1.9.3-x86_64.exe",
        "master/sctgdesk-releases/sctgdesk-1.2.6.dmg",
        "master/sctgdesk-releases/sctgdesk-1.2.6.ipa",
        "master/sctgdesk-releases/sctgdesk-1.4.0-web-basic.zip",
        "master/sctgdesk-releases/sctgdesk-1.4.0-web-basic.tar.gz",
        "master/sctgdesk-releases/sctgdesk-1.5.0-rc1.dmg",
    ];

    fn latest(template: &str) -> Option<(String, String)> {
        pick_latest(template, KEYS.iter().copied()).map(|r| (r.key, r.version.unwrap()))
    }

    #[test]
    fn the_highest_version_wins_numerically_not_alphabetically() {
        let (key, version) =
            latest("master/sctgdesk-releases/sctgdesk-{version}-x86_64.exe").unwrap();
        assert_eq!(version, "1.10.0");
        assert_eq!(key, "master/sctgdesk-releases/sctgdesk-1.10.0-x86_64.exe");
    }

    #[test]
    fn only_keys_that_match_the_whole_template_count() {
        // not i686, not the web archives, not a pre-release, not the key pair
        assert_eq!(
            latest("master/sctgdesk-releases/sctgdesk-{version}.dmg")
                .unwrap()
                .1,
            "1.2.6"
        );
        assert_eq!(
            latest("master/sctgdesk-releases/sctgdesk-{version}.ipa")
                .unwrap()
                .1,
            "1.2.6"
        );
        assert_eq!(
            latest("master/sctgdesk-releases/sctgdesk-{version}-i686.exe")
                .unwrap()
                .1,
            "1.2.6"
        );
        assert_eq!(
            latest("master/sctgdesk-releases/sctgdesk-{version}-web-basic.zip")
                .unwrap()
                .1,
            "1.4.0"
        );
    }

    #[test]
    fn nothing_matches_nothing() {
        assert_eq!(
            latest("master/sctgdesk-releases/sctgdesk-{version}.apk"),
            None
        );
        assert_eq!(latest("master/other/sctgdesk-{version}.dmg"), None);
        // no placeholder: not a template
        assert_eq!(latest("master/sctgdesk-releases/sctgdesk-1.2.6.dmg"), None);
        assert_eq!(
            pick_latest("master/x-{version}.dmg", std::iter::empty()),
            None
        );
    }

    #[test]
    fn versions_are_three_numbers() {
        assert_eq!(parse_version("1.5.0"), Some([1, 5, 0]));
        assert_eq!(parse_version("10.20.30"), Some([10, 20, 30]));
        for bad in [
            "",
            "1.5",
            "1.5.0.1",
            "1.5.0-rc1",
            "web",
            "1..0",
            "+1.5.0",
            "1.5.-0",
        ] {
            assert_eq!(parse_version(bad), None, "{bad:?}");
        }
    }

    fn config(list_bucket: Option<&str>) -> S3Config {
        S3Config {
            page: String::new(),
            endpoint: "https://compat.objectstorage.eu-marseille-1.oraclecloud.com".into(),
            region: "eu-marseille-1".into(),
            access_key: "ak".into(),
            secret_key: "sk".into(),
            bucket: "axzuz14sllpf".into(),
            windows64_key: String::new(),
            windows32_key: String::new(),
            osxkey: String::new(),
            osxarm64_key: String::new(),
            ioskey: String::new(),
            list_bucket: list_bucket.map(String::from),
        }
    }

    #[test]
    fn listing_uses_the_bucket_as_it_is_by_default() {
        let target = listing_target(&config(None), "master/sctgdesk-releases/sctgdesk-").unwrap();
        assert_eq!(
            target.endpoint,
            "https://compat.objectstorage.eu-marseille-1.oraclecloud.com"
        );
        assert_eq!(target.bucket, "axzuz14sllpf");
        assert!(!target.path_style);
        assert_eq!(target.lead, "");
        assert_eq!(target.prefix, "master/sctgdesk-releases/sctgdesk-");
    }

    #[test]
    fn listing_with_list_bucket_puts_the_namespace_in_the_host_and_the_bucket_in_the_path() {
        let target = listing_target(
            &config(Some("master")),
            "master/sctgdesk-releases/sctgdesk-",
        )
        .unwrap();
        assert_eq!(
            target.endpoint,
            "https://axzuz14sllpf.compat.objectstorage.eu-marseille-1.oraclecloud.com"
        );
        assert_eq!(target.bucket, "master");
        assert!(target.path_style);
        assert_eq!(target.lead, "master/");
        assert_eq!(target.prefix, "sctgdesk-releases/sctgdesk-");
        // a key outside the bucket is an error, not a listing of something else
        assert!(listing_target(&config(Some("master")), "other/sctgdesk-").is_err());
    }

    #[cfg(feature = "aws")]
    #[tokio::test]
    async fn a_key_without_placeholder_is_used_as_it_is_without_asking_the_bucket() {
        let cfg = Config {
            s3config: config(None),
        };
        let resolved = resolve_key(&cfg, "master/sctgdesk-releases/sctgdesk-1.2.6-x86_64.exe")
            .await
            .unwrap();
        assert_eq!(
            resolved.key,
            "master/sctgdesk-releases/sctgdesk-1.2.6-x86_64.exe"
        );
        assert_eq!(resolved.version.as_deref(), Some("1.2.6"));
        let no_version = resolve_key(&cfg, "master/sctgdesk-releases/latest.bin")
            .await
            .unwrap();
        assert_eq!(no_version.version, None);
    }

    #[cfg(feature = "aws")]
    /// Run on a host that has the real s3config.toml (S3CONFIG_FILE):
    /// `cargo test -p s3software live_latest -- --ignored --nocapture`
    #[tokio::test]
    #[ignore = "lists the live bucket described by s3config.toml"]
    async fn live_latest_keys() {
        let config = get_s3_config_file().await.unwrap();
        for template in [
            &config.s3config.windows64_key,
            &config.s3config.osxkey,
            &config.s3config.ioskey,
        ] {
            let resolved = resolve_key(&config, template).await.unwrap();
            println!("{template} -> {} ({:?})", resolved.key, resolved.version);
        }
    }
}
