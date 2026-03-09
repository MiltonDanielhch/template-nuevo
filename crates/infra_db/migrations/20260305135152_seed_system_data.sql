-- 20260305135152_seed_system_data.sql
-- c:\laravel\templates\template1\crates\infra_db\migrations\20260305135152_seed_system_data.sql

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
