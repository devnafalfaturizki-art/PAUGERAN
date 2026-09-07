CREATE TABLE IF NOT EXISTS cases (
    id TEXT PRIMARY KEY NOT NULL,
    title TEXT NOT NULL,
    state TEXT NOT NULL,
    mode TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS messages (
    id TEXT PRIMARY KEY NOT NULL,
    case_id TEXT NOT NULL,
    role TEXT NOT NULL,
    content TEXT NOT NULL,
    certainty_score REAL,
    created_at TEXT NOT NULL,
    FOREIGN KEY (case_id) REFERENCES cases(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_messages_case_created
    ON messages(case_id, created_at);
