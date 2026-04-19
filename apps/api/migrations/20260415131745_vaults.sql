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
	encrypted_vault_key TEXT NOT NULL,

	PRIMARY KEY (vault_id, user_id)
);

CREATE INDEX idx_vault_users_user_id ON vaults_users(user_id);
