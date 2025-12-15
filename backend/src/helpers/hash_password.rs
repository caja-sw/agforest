use argon2::{
    Argon2,
    password_hash::{PasswordHasher, Result, SaltString, rand_core::OsRng},
};

pub fn hash_password(password: &[u8]) -> Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    argon2
        .hash_password(password, &salt)
        .map(|hash| hash.to_string())
}
