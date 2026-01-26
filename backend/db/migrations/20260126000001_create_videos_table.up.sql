CREATE TABLE IF NOT EXISTS videos (
    id TEXT PRIMARY KEY NOT NULL,
    youtube_url TEXT NOT NULL UNIQUE,
    title TEXT,
    duration_seconds INTEGER,
    is_active INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_videos_is_active ON videos (is_active);
CREATE INDEX IF NOT EXISTS idx_videos_created_at ON videos (created_at);
