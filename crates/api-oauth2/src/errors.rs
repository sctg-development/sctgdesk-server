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
use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub enum Oauth2Error {
    ExchangeCodeError,
    VerifyTokenError,
    DecodeIdTokenError,
}

impl fmt::Display for Oauth2Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Oauth2Error::ExchangeCodeError => write!(f, "Exchange code error"),
            Oauth2Error::VerifyTokenError => write!(f, "Verify token error"),
            Oauth2Error::DecodeIdTokenError => write!(f, "Decode id token error"),
        }
    }
}

impl Error for Oauth2Error {}