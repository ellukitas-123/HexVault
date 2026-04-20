-- Add migration script here
ALTER TABLE users
ADD COLUMN nonce VARCHAR(255) NOT NULL;