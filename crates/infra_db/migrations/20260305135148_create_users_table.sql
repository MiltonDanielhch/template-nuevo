-- 20260305135148_create_users_table.sql
-- Migración inicial para la tabla de usuarios

CREATE TABLE IF NOT EXISTS users (
    id BLOB PRIMARY KEY NOT NULL, -- UUIDv7 se almacena como BLOB en SQLite
    email TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);