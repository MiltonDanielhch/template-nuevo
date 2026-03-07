-- 20260305135148_create_users_table.sql
-- c:\laravel\templates\template1\crates\infra_db\migrations\20260305135148_create_users_table.sql
--! # Migración: Create Users Table
--!
--! Crea la tabla `users` inicial, que es el pilar del sistema de autenticación.
--!
--! ## Decisiones de Diseño (Sintonía 3026)
--! - `id TEXT`: Usamos `TEXT` para almacenar UUIDs (v7) para máxima compatibilidad.
--! - `deleted_at DATETIME`: Implementa el patrón "Soft Delete". Los usuarios no se borran, se marcan como borrados.
--! - `idx_users_email_active`: Un índice único PARCIAL. Permite que un nuevo usuario se registre con un email
--!   que pertenecía a una cuenta borrada, pero no permite emails duplicados entre usuarios activos.
--! - `trg_users_updated_at`: Un `TRIGGER` que actualiza automáticamente el campo `updated_at` en cada modificación.
--!   Esto descarga a la lógica de aplicación de esta responsabilidad, garantizando la consistencia.

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

-- Tabla intermedia para Usuarios y Roles (SOPORTE ESCALABILIDAD N:M)
CREATE TABLE IF NOT EXISTS user_roles (
    user_id TEXT NOT NULL,
    role_id TEXT NOT NULL,
    assigned_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (user_id, role_id),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (role_id) REFERENCES roles(id) ON DELETE CASCADE
);
