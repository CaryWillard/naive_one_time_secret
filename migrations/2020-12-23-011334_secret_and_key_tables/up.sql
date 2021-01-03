CREATE TABLE secret_keys (
  id INT GENERATED ALWAYS AS IDENTITY,
  secret_key BYTEA NOT NULL,
  PRIMARY KEY(id)
);

CREATE TABLE secrets (
    id INT GENERATED ALWAYS AS IDENTITY,
    key_id INT NOT NULL,
    hash BYTEA NOT NULL,
    nonce BYTEA NOT NULL,
    secret BYTEA NOT NULL,
    PRIMARY KEY(id),
    CONSTRAINT fk_secret_keys
          FOREIGN KEY(key_id)
    	  REFERENCES secret_keys(id)
);