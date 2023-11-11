extern crate bcrypt;

use bcrypt::{DEFAULT_COST, hash, verify};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub email: String,
    password_hash: String,
}

impl User {
    fn new(username: String, email: String, password: String) -> Self {
        return User {
            username,
            email,
            password_hash: hash(password, DEFAULT_COST).unwrap(),
        }
    }
    fn check_password(self, pass: String) -> bool {
        let valid = verify(pass, self.password_hash.as_str());
        return valid.unwrap();
    }
}