export type NodeType = 'case' | 'party' | 'fact' | 'evidence' | 'issue' | 'rule' | 'source' | 'argument' | 'counterargument' | 'risk' | 'conclusion' | 'document';
export type EdgeType = 'supports' | 'challenges' | 'governed_by' | 'derived_from' | 'uncertain_because' | 'supported_by' | 'relies_on' | 'sourced_from';

export interface CaseGraphNode {
  id: string;
  caseId: string;
  nodeType: NodeType;
  content: string;
  metadata: Record<string, unknown>;
  createdAt: string;
}

export interface CaseGraphEdge {
  id: string;
  caseId: string;
  sourceNodeId: string;
  targetNodeId: string;
  edgeType: EdgeType;
  metadata: Record<string, unknown>;
  createdAt: string;
}
