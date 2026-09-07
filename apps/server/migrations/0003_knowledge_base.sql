CREATE TABLE IF NOT EXISTS knowledge_documents (
    id TEXT PRIMARY KEY NOT NULL,
    title TEXT NOT NULL,
    full_text TEXT NOT NULL,
    citation TEXT NOT NULL,
    metadata TEXT NOT NULL DEFAULT '{}',
    created_at TEXT NOT NULL
);
CREATE INDEX IF NOT EXISTS idx_knowledge_documents_title ON knowledge_documents(title);
