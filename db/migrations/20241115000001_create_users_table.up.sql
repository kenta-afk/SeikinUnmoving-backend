-- Create users table
-- This migration creates the users table for storing user information
-- Compatible with both SQLite (development) and Cloudflare D1 (production)

CREATE TABLE IF NOT EXISTS users (
    id TEXT PRIMARY KEY NOT NULL,  -- UUID v7
    username TEXT NOT NULL UNIQUE,
    email TEXT NOT NULL UNIQUE,
    password TEXT NOT NULL,  -- Hashed password
    created_at TEXT NOT NULL,  -- ISO8601 timestamp
    updated_at TEXT NOT NULL   -- ISO8601 timestamp
);

-- Create indexes for frequently queried columns
CREATE INDEX IF NOT EXISTS idx_users_email ON users(email);
CREATE INDEX IF NOT EXISTS idx_users_username ON users(username);
