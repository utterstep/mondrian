use std::env;

use argon2::{self, Config, Variant};
use lazy_static::lazy_static;

lazy_static! {
    static ref CONFIG: Config<'static> = {
        let mut config = Config::default();
        config.variant = Variant::Argon2id;

        config
    };
}

lazy_static! {
    static ref SALT: Vec<u8> = {
        let salt = env::var("MONDRIAN_SALT").unwrap();
        salt.into_bytes()
    };
}

pub fn hash_password(plain: &str) -> argon2::Result<String> {
    hash_password_inner(plain, &SALT)
}

fn hash_password_inner(plain: &str, salt: &[u8]) -> argon2::Result<String> {
    argon2::hash_encoded(plain.as_bytes(), salt, &CONFIG)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_SALT: &[u8] = b"TEST SALT";
    const TEST_SALT_TWO: &[u8] = b"OTHER TEST SALT";

    #[test]
    fn test_hash_password() {
        let password1 = "some password";
        let password2 = "other password";

        let hash1 = hash_password_inner(password1, TEST_SALT);
        let hash2 = hash_password_inner(password2, TEST_SALT);

        assert!(hash1.is_ok());
        assert!(hash2.is_ok());

        let hash1 = hash1.unwrap();
        let hash2 = hash2.unwrap();
        assert_ne!(hash1, hash2);

        let hash1_other_salt = hash_password_inner(password1, TEST_SALT_TWO);
        let hash2_other_salt = hash_password_inner(password2, TEST_SALT_TWO);

        assert!(hash1_other_salt.is_ok());
        assert!(hash2_other_salt.is_ok());

        let hash1_other_salt = hash1_other_salt.unwrap();
        let hash2_other_salt = hash2_other_salt.unwrap();

        assert_ne!(hash1_other_salt, hash2_other_salt);

        assert_ne!(hash1, hash1_other_salt);
        assert_ne!(hash2, hash2_other_salt);
    }
}
