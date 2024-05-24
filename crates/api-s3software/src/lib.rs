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
use aws_config::{BehaviorVersion, ConfigLoader};
use aws_sdk_s3::presigning::PresigningConfig;
use aws_sdk_s3::{config::Region, Client};
use serde::Deserialize;
use std::error::Error;
use std::fs;
use std::time::Duration;

#[derive(Deserialize, Debug, Clone)]
pub struct S3Config {
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
}

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub s3config: S3Config,
}
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

pub async fn get_signed_release_url(
    endpoint: &str,
    region: &str,
    bucket: &str,
    object: &str,
    expires_in: u64,
) -> Result<String, Box<dyn Error>> {
    let region = region.to_string();
    let region_provider = Region::new(region);

    let shared_config = ConfigLoader::default().behavior_version(BehaviorVersion::latest())
        .region(region_provider)
        .endpoint_url(endpoint)
        .load()
        .await;
    let client = Client::new(&shared_config);
    get_object(&client, &bucket, &object, expires_in).await
}

pub async fn parse_config(config: &str) -> Result<Config, Box<dyn Error>> {
    let config = toml::from_str(config);
    if config.is_err() {
        return Err("Error parsing config".into());
    }
    Ok(config.unwrap())
}

/// Get the name of the s3 config file
/// from the S3COONFIG_FILE environment variable or
/// default to "s3config.toml"
///
/// # Returns
/// The s3config
pub async fn get_s3_config_file() -> Result<Config, Box<dyn Error>> {
    let config_filename =
        std::env::var("S3CONFIG_FILE").unwrap_or_else(|_| "s3config.toml".to_string());
    // If file does not exist create it
    if !std::path::Path::new(&config_filename).exists() {
        log::error!("S3 config file does not exist, creating it, we recommend you to fill it with your own values, you can change the file path by setting the S3CONFIG_FILE environment variable.");
        let s3_config = include_str!("../../../s3config.toml");
        fs::write(&config_filename, s3_config).expect("Failed to write s3 config file");
    }
    let config_file_content =
        fs::read_to_string(config_filename).expect("Failed to read s3 config file");
    parse_config(&config_file_content).await
}

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

#[cfg(test)]
mod tests {
    use super::*;
    use reqwest::StatusCode;

    #[tokio::test]
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
