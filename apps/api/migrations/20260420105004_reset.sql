CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email VARCHAR(255) UNIQUE NOT NULL,
    master_password_hash VARCHAR(255) NOT NULL,
	salt BYTEA NOT NULL,
	public_key BYTEA NOT NULL,
	encrypted_private_key BYTEA NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
	updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
	
	CONSTRAINT users_salt_len_check CHECK (octet_length(salt) = 16),
	CONSTRAINT users_public_key_len_check CHECK (octet_length(public_key) = 32),
	CONSTRAINT users_encrypted_private_key_len_check CHECK (octet_length(encrypted_private_key) = 60)
);

-- We will be looking for users by email
CREATE INDEX idx_users_email ON users(email);

CREATE TABLE vaults (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
	owner UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
	color VARCHAR(255) NOT NULL,
	created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE vaults_users (
	user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
	vault_id UUID NOT NULL REFERENCES vaults(id) ON DELETE CASCADE,
	user_role VARCHAR(20) NOT NULL,
	encrypted_vault_key BYTEA NOT NULL,

	PRIMARY KEY (vault_id, user_id),
	CONSTRAINT vaults_users_encrypted_vault_key_len_check CHECK (octet_length(encrypted_vault_key) = 60)
);

CREATE INDEX idx_vault_users_user_id ON vaults_users(user_id);