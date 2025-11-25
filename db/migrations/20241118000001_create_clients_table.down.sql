-- Rollback migration for clients table
-- This will drop the clients table and its indexes

DROP INDEX IF EXISTS idx_clients_exp;
DROP INDEX IF EXISTS idx_clients_jti;
DROP INDEX IF EXISTS idx_clients_user_id;
DROP TABLE IF EXISTS clients;
