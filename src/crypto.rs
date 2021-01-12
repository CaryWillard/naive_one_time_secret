use super::models::*;
use super::error;
use sodiumoxide::crypto::secretbox;
use sodiumoxide::crypto::hash::sha256;

pub fn encrypt_secret(plaintext: &str) -> Result<InsertableSecret, error::Error> {
    let key = get_key()?;
    let nonce = secretbox::gen_nonce();
    let secret = secretbox::seal(plaintext.as_bytes(), &nonce, &key);
    let hash = sha256::hash(&secret);

    let result = InsertableSecret {
        key_id: 1,
        hash: hash.as_ref().to_vec(),
        nonce: nonce.as_ref().to_vec(),
        secret,
    };

    Ok(result)
}

fn get_key() -> Result<secretbox::xsalsa20poly1305::Key, error::Error> {
    // Try to get the first key from the database
    // If there is none, make a new one
    Ok(secretbox::gen_key())
    // Insert the new key
    // Return the new key
}

pub fn decrypt_secret(secret: &Secret, secret_key: &SecretKey) -> Result<String, error::Error> {
    let result = secretbox::open(&secret.secret, &secret.get_nonce()?, &secret_key.get_key()?);
    match result {
        Ok(plaintext_bytes) => Ok(String::from_utf8(plaintext_bytes)?),
        Err(_) => Err(error::Error::SecretboxOpen(error::Msg::new("Failed to open a secretbox"))),
    }
}

fn get_key_by_id(_key_id: i32) -> Result<secretbox::xsalsa20poly1305::Key, error::Error> {
    unimplemented!()
}