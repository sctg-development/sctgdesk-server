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
use rand::{thread_rng, Rng};
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use base64::prelude::{Engine as _, BASE64_URL_SAFE_NO_PAD};
const TOKEN_LENGTH: usize = 32;

#[must_use]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, JsonSchema)]
pub struct Token([u8; TOKEN_LENGTH]);

impl Token {
    pub fn new_random() -> Self {
        let mut random_bytes = [0u8; TOKEN_LENGTH];
        thread_rng().fill(&mut random_bytes);
        Self(random_bytes)
    }

    /// Convert into base64.
    pub fn to_base64(&self) -> String {
        BASE64_URL_SAFE_NO_PAD.encode(&self.0)
    }

    pub fn from_str<S: AsRef<str>>(str: S) -> Result<Self, base64::DecodeError> {
        let bytes = BASE64_URL_SAFE_NO_PAD.decode(str.as_ref()).unwrap();
        let mut buf = [0u8; TOKEN_LENGTH];
        buf.copy_from_slice(&bytes);
        Ok(Self(buf))
    }
}

impl serde::Serialize for Token {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.to_base64().serialize(serializer)
    }
}

impl<'de> serde::Deserialize<'de> for Token {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let token = Self::from_str(&s).map_err(serde::de::Error::custom)?;
        Ok(token)
    }
}

