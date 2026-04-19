-- RENAME email_hash to email and update the index accordingly
ALTER TABLE users RENAME COLUMN email_hash TO email;
DROP INDEX idx_users_email_hash;
CREATE INDEX idx_users_email ON users(email);
