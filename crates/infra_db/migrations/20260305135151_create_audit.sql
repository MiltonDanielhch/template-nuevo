CREATE TABLE IF NOT EXISTS audit_logs (
    id TEXT PRIMARY KEY NOT NULL,
    user_id TEXT, 
    action TEXT NOT NULL, 
    resource TEXT NOT NULL, 
    resource_id TEXT, 
    payload TEXT,
    ip_address TEXT,
    user_agent TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE SET NULL
);

CREATE INDEX IF NOT EXISTS idx_audit_resource_search ON audit_logs(resource, resource_id);
CREATE INDEX IF NOT EXISTS idx_audit_user_history ON audit_logs(user_id, created_at);
