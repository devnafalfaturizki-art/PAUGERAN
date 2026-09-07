export interface KnowledgeEntry {
  id?: string;
  title: string;
  fullText: string;
  hierarchyLevel: number;
  precedentialWeight?: number;
  effectiveDate?: string;
  revokedDate?: string;
  tags: string[];
  sourceUrl?: string;
  createdAt?: string;
}

export interface KnowledgeSearchResult {
  entry: KnowledgeEntry;
  score: number;
}
