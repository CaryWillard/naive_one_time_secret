use super::schema::{secrets, secret_keys};
use sodiumoxide::crypto::secretbox::Nonce;
use super::error::{Error, Msg};

#[derive(Identifiable)]
#[derive(Queryable)]
pub struct Secret {
    pub id: i32,
    pub key_id: i32,
    pub hash: Vec<u8>,
    pub nonce: Vec<u8>,
    pub secret: Vec<u8>,
}

impl Secret {
    pub fn get_nonce(&self) -> Result<Nonce, Error> {
        let nonce = Nonce::from_slice(&self.nonce)
            .ok_or_else(|| Error::None(Msg::new("Failed to get nonce from secret record")))?;
        Ok(nonce)
    }
}

#[derive(Insertable)]
#[table_name = "secrets"]
pub struct InsertableSecret {
    pub key_id: i32,
    pub hash: Vec<u8>,
    pub nonce: Vec<u8>,
    pub secret: Vec<u8>,
}

#[derive(Identifiable)]
#[derive(Queryable)]
pub struct SecretKey {
    pub id: i32,
    pub secret_key: Vec<u8>,
}

#[derive(Insertable)]
#[table_name = "secret_keys"]
pub struct InsertableSecretKey {
    pub secret_key: Vec<u8>,
}