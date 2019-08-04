use argon2;
use chrono::NaiveDateTime;
use serde::Deserialize;

use crate::{schema::users, utils::crypto};

#[derive(Debug, Deserialize, Queryable, Insertable, Identifiable)]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub middle_name: String,
    pub email: String,
    pub phone: String,
    password: String,
    pub superuser: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name = "users"]
pub struct NewUserEncrypted {
    pub first_name: String,
    pub last_name: String,
    pub middle_name: String,
    pub email: String,
    pub phone: String,
    password: String,
    pub superuser: bool,
}

#[derive(Debug, Deserialize)]
pub struct NewUserPlain {
    pub first_name: String,
    pub last_name: String,
    pub middle_name: String,
    pub email: String,
    pub phone: String,
    password: Option<String>,
}

impl User {
    pub fn check_password(&self, password: &str) -> argon2::Result<bool> {
        argon2::verify_encoded(&self.password, password.as_bytes())
    }
}

impl NewUserEncrypted {
    pub fn from_plain(user: NewUserPlain) -> argon2::Result<Self> {
        let password = if let Some(password) = user.password {
            crypto::hash_password(&password)?
        } else {
            "".to_owned()
        };

        Ok(Self {
            first_name: user.first_name,
            last_name: user.last_name,
            middle_name: user.middle_name,
            email: user.email,
            phone: user.phone,
            password,
            superuser: false,
        })
    }
}
