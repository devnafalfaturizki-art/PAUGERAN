CREATE TABLE IF NOT EXISTS case_nodes (
    id TEXT PRIMARY KEY NOT NULL,
    case_id TEXT NOT NULL,
    node_type TEXT NOT NULL,
    content TEXT NOT NULL,
    metadata TEXT NOT NULL DEFAULT '{}',
    created_at TEXT NOT NULL,
    FOREIGN KEY (case_id) REFERENCES cases(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS case_edges (
    id TEXT PRIMARY KEY NOT NULL,
    case_id TEXT NOT NULL,
    source_node_id TEXT NOT NULL,
    target_node_id TEXT NOT NULL,
    edge_type TEXT NOT NULL,
    metadata TEXT NOT NULL DEFAULT '{}',
    created_at TEXT NOT NULL,
    FOREIGN KEY (case_id) REFERENCES cases(id) ON DELETE CASCADE,
    FOREIGN KEY (source_node_id) REFERENCES case_nodes(id) ON DELETE CASCADE,
    FOREIGN KEY (target_node_id) REFERENCES case_nodes(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_case_nodes_case_type
    ON case_nodes(case_id, node_type);
CREATE INDEX IF NOT EXISTS idx_case_edges_case
    ON case_edges(case_id);
