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
mod tokens;
mod bearer;
pub mod address_book;

pub mod types;
pub mod get_host;

pub use tokens::Token;
pub use bearer::{BearerAuthToken, CookieAuthToken, MixedAuthToken, IntoToken};
// pub use address_book::AddressBook;
pub use types::*;
use uuid::Uuid;

#[macro_export]
macro_rules! unwrap_or_return {
    ($optval:expr) => {
        match $optval {
            Ok(val) => val,
            Err(err) => {
                log::debug!("ERR in unwrap_or_return: {:?}", &err);
                return err;
            },
        }
    };
}

#[macro_export]
macro_rules! include_svg_as_base64 {
    ($path:expr) => {
        {
            use ::base64::prelude::{Engine as _, BASE64_STANDARD};
            let svg_data = include_bytes!($path);
            let base64 = BASE64_STANDARD.encode(svg_data);
            format!("data:image/svg+xml;base64,{}", base64)
        }
    };
}

#[macro_export]
macro_rules! include_png_as_base64 {
    ($path:expr) => {
        {
            use ::base64::prelude::{Engine as _, BASE64_STANDARD};
            let svg_data = include_bytes!($path);
            let base64 = BASE64_STANDARD.encode(svg_data);
            format!("data:image/png;base64,{}", base64)
        }
    };
}

pub fn guid_into_uuid(guid: Vec<u8>) -> Option<String> {
    let guid_u8: Result<[u8; 16], _> = guid.try_into();
    if guid_u8.is_err() {
        log::error!("get_ab_personal_guid error: {:?}", guid_u8);
        return None;
    }
    let guid_u8: [u8; 16] = guid_u8.unwrap();
    let guid = Uuid::from_bytes(guid_u8).to_string();
    Some(guid)
}

pub fn uuid_into_guid(uuid: &str) -> Option<Vec<u8>> {
    let uuid = Uuid::parse_str(uuid);
    if uuid.is_err() {
        log::error!("uuid_into_guid error: {:?}", uuid);
        return None;
    }
    let uuid = uuid.unwrap();
    let guid = uuid.as_bytes().to_vec();
    Some(guid)
}