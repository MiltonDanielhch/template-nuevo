-- 20260305135152_seed_system_data.sql
-- c:\laravel\templates\template1\crates\infra_db\migrations\20260305135152_seed_system_data.sql

-- Usuarios iniciales (Admin predeterminado)
-- Password: 12345678
INSERT OR IGNORE INTO users (id, username, email, password_hash, email_verified) VALUES
('user_admin_3026', 'admin', 'admin@admin.com', '$argon2id$v=19$m=19456,t=2,p=1$BTRPpS9b6cs0xopRihl7m5a2E', 1);

-- Roles iniciales
INSERT OR IGNORE INTO roles (id, name, description) VALUES
('role_00000001', 'Admin', 'Acceso total al sistema'),
('role_00000002', 'User', 'Acceso estándar de usuario');

-- Permisos iniciales
INSERT OR IGNORE INTO permissions (id, name, description) VALUES
('perm_00000001', 'users:read', 'Ver lista de usuarios'),
('perm_00000002', 'users:write', 'Crear y editar usuarios'),
('perm_00000003', 'audit:read', 'Ver logs de auditoría'),
('perm_00000004', 'roles:read', 'Ver roles y permisos'),
('perm_00000005', 'roles:write', 'Gestionar roles y permisos');

-- Asignar TODOS los permisos al rol Admin automáticamente
INSERT OR IGNORE INTO role_permissions (role_id, permission_id)
SELECT r.id, p.id FROM roles r CROSS JOIN permissions p
WHERE r.name = 'Admin';

-- Asegurar que el usuario admin tenga el rol Admin
INSERT OR IGNORE INTO user_roles (user_id, role_id)
SELECT u.id, r.id FROM users u CROSS JOIN roles r
WHERE u.username = 'admin' AND r.name = 'Admin';
