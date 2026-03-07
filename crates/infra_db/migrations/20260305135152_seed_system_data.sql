INSERT OR IGNORE INTO roles (id, name, description) VALUES 
('role_00000001', 'Admin', 'Acceso total'), 
('role_00000002', 'User', 'Acceso estándar');

INSERT OR IGNORE INTO permissions (id, name, description) VALUES 
('perm_00000001', 'users:read', 'Ver usuarios'),
('perm_00000002', 'users:write', 'Editar usuarios'),
('perm_00000003', 'audit:read', 'Ver logs');

INSERT OR IGNORE INTO role_permissions (role_id, permission_id) 
SELECT r.id, p.id FROM roles r CROSS JOIN permissions p WHERE r.name = 'Admin';
