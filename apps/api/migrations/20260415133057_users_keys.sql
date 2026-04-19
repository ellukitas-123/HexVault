ALTER TABLE users
ADD COLUMN public_key TEXT NOT NULL, -- Public key to encrypt vault keys for the user
ADD COLUMN encrypted_private_key TEXT NOT NULL -- Encrypted private key for the user, encrypted with their password
;