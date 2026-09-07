CREATE TABLE IF NOT EXISTS knowledge_embeddings (
    document_id TEXT PRIMARY KEY NOT NULL,
    embedding TEXT NOT NULL,
    dimensions INTEGER NOT NULL,
    created_at TEXT NOT NULL,
    FOREIGN KEY (document_id) REFERENCES knowledge_documents(id) ON DELETE CASCADE
);
