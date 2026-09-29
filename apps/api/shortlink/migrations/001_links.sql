CREATE TABLE IF NOT EXISTS links (
    code TEXT PRIMARY KEY,
    url TEXT NOT NULL,
    hits INTEGER NOT NULL DEFAULT 0,
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
);
