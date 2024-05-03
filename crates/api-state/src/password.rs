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

