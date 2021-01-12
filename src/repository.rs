use crate::models::*;
use crate::error;
use crate::schema::*;
use diesel::prelude::*;
use std::env;
use diesel::insert_into;
use dotenv::dotenv;
use crate::schema::secret_keys::columns::secret_key;

pub struct Repo {
    connection: PgConnection,
}

impl Repo {
    pub fn new() -> Repo {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");
        let connection = PgConnection::establish(&database_url)
            .expect(&format!("Error connecting to {}", database_url));

        Repo {
            connection,
        }
    }

    pub fn store_key(&self, key: &InsertableSecretKey) -> Result<i32, error::Error> {
        use secret_keys::dsl::id;

        let result = insert_into(secret_keys::table)
            .values(vec![key])
            .returning(id)
            .execute(&self.connection)?;

        Ok(result as i32)
    }

    pub fn get_key(&self, id_of_key: i32) -> Result<SecretKey, error::Error> {
        use secret_keys::dsl::id;

        let result = secret_keys::table
            .filter(id.eq(id_of_key))
            .first(&self.connection)?;

        Ok(result)
    }

    pub fn store_secret(&self, new_secret: &InsertableSecret) -> Result<(), error::Error> {
        insert_into(secrets::table)
            .values(new_secret)
            .execute(&self.connection)?;

        Ok(())
    }

    pub fn get_secret(&self, query_hash: &[u8]) -> Result<Secret, error::Error> {
        use secrets::dsl::hash;

        let result = secrets::table
            .filter(hash.eq(query_hash))
            .first::<Secret>(&self.connection)?;

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn stores_and_reads_key() -> Result<(), error::Error> {
        let repo = Repo::new();

        let key = InsertableSecretKey {
            secret_key: b"test key".to_vec()
        };

        let id = repo.store_key(&key)?;
        let result_key = repo.get_key(id)?;

        assert_eq!(key.secret_key, result_key.secret_key);

        Ok(())
    }

    #[test]
    #[ignore]
    fn stores_and_reads_secret() -> Result<(), error::Error> {
        let repo = Repo::new();

        let hash = b"Test hash".to_vec();
        let key_id = 1;

        let secret = InsertableSecret {
            key_id,
            secret: b"Test secret".to_vec(),
            nonce: b"Test nonce".to_vec(),
            hash: hash.clone(),
        };

        repo.store_secret(&secret)?;
        let result_secret = repo.get_secret(&hash)?;

        assert_eq!(secret.secret, result_secret.secret);
        assert_eq!(secret.hash, result_secret.hash);
        assert_eq!(secret.nonce, result_secret.nonce);

        Ok(())
    }
}