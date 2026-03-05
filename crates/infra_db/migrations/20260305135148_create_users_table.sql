-- 20260305135148_create_users_table.sql
-- Migración inicial para la tabla de usuarios (versión con soft-delete y trigger)

CREATE TABLE IF NOT EXISTS users (
    id TEXT PRIMARY KEY NOT NULL, -- UUID v4/v7 como texto para flexibilidad
    username TEXT,
    email TEXT NOT NULL,
    password_hash TEXT NOT NULL,
    avatar_url TEXT,
    email_verified BOOLEAN NOT NULL DEFAULT FALSE,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at DATETIME
);

-- Índice parcial: Permite re-usar el email si la cuenta anterior fue "borrada" (Soft Delete)
CREATE UNIQUE INDEX IF NOT EXISTS idx_users_email_active 
ON users(email) WHERE deleted_at IS NULL;

-- Trigger para auto-actualizar updated_at en la tabla users
CREATE TRIGGER IF NOT EXISTS trg_users_updated_at
AFTER UPDATE ON users
FOR EACH ROW
BEGIN
    UPDATE users SET updated_at = CURRENT_TIMESTAMP WHERE id = OLD.id;
END;