use argon2::password_hash::{PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::{password_hash::SaltString, Argon2};
use rand::rngs::StdRng;
use rand::SeedableRng;

use crate::error::AppError;

pub fn hash_password(password: &str, pepper: &str) -> Result<String, AppError> {
    let salted = format!("{}{}", pepper, password);
    let argon2 = Argon2::default();
    let mut rng = StdRng::from_entropy();
    let salt = SaltString::generate(&mut rng);
    let hash = argon2
        .hash_password(salted.as_bytes(), &salt)
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;
    Ok(hash.to_string())
}

pub fn verify_password(password: &str, pepper: &str, hash: &str) -> Result<bool, AppError> {
    let salted = format!("{}{}", pepper, password);
    let parsed_hash =
        PasswordHash::new(hash).map_err(|e| AppError::InternalServerError(e.to_string()))?;
    let argon2 = Argon2::default();
    Ok(argon2
        .verify_password(salted.as_bytes(), &parsed_hash)
        .is_ok())
}
