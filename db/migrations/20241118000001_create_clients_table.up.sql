CREATE TABLE IF NOT EXISTS clients (
    client_id TEXT PRIMARY KEY NOT NULL,
    user_id TEXT NOT NULL,
    jti TEXT NOT NULL UNIQUE,
    exp INTEGER NOT NULL,
    created_at TEXT NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_clients_user_id ON clients (user_id);
CREATE INDEX IF NOT EXISTS idx_clients_jti ON clients (jti);
CREATE INDEX IF NOT EXISTS idx_clients_exp ON clients (exp);
