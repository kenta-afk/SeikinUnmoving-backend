-- Rollback migration for users table
-- This will drop the users table and its indexes

DROP INDEX IF EXISTS idx_users_email;
DROP TABLE IF EXISTS users;
