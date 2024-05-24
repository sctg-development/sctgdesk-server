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
use crate::database::DatabaseUserPasswordInfo;
use bcrypt::{hash, verify, DEFAULT_COST};

pub struct UserPasswordInfo<'s> {
    password: &'s str,
}

impl<'s> UserPasswordInfo<'s> {
    pub fn from_password( password: &'s str ) -> Self {
        Self { 
            password
        }
    }

    pub fn check( &self, db_password_info: DatabaseUserPasswordInfo ) -> bool {
        let is_valid = verify(self.password, db_password_info.password.as_str()).unwrap();
        is_valid
    }

    pub fn check_with_string( &self, password_string: String) -> bool {
        let hashed_password = hash(self.password, DEFAULT_COST).unwrap();
        let is_valid = verify(password_string, &hashed_password).unwrap();
        is_valid
    }

    pub fn hash_password( given_password: &str) -> String {
        hash(given_password, DEFAULT_COST).unwrap()
    }

}

