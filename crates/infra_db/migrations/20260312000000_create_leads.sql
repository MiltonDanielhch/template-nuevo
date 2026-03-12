-- 20260312000000_create_leads.sql
-- c:\laravel\templates\template1\crates\infra_db\migrations\20260312000000_create_leads.sql
--! # Migración: Create Leads Table
--!
--! Crea la tabla `leads`, utilizada para almacenar los registros de contactos
--! capturados desde la landing page.
--!
--! ## Decisiones de Diseño
--! - `id TEXT`: UUIDv7 (ordenable) para garantizar unicidad en la base de datos.
--! - `email TEXT`: Se indexa para búsquedas rápidas y para evitar duplicados.
--! - `created_at/updated_at`: Timestamps para auditoría básica.
--! - `trg_leads_updated_at`: Trigger para mantener `updated_at` en cada modificación.

CREATE TABLE IF NOT EXISTS leads (
    id TEXT PRIMARY KEY NOT NULL,
    email TEXT NOT NULL,
    name TEXT,
    source TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_leads_email
ON leads(email);

CREATE TRIGGER IF NOT EXISTS trg_leads_updated_at
AFTER UPDATE ON leads
FOR EACH ROW
BEGIN
    UPDATE leads SET updated_at = CURRENT_TIMESTAMP WHERE id = OLD.id;
END;
