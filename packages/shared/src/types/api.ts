import type { AnalysisResponse } from './reasoning';
import type { CaseGraphEdge, CaseGraphNode } from './case_graph';

export interface CreateCaseRequest { title: string }
export interface CaseGraphResponse { nodes: CaseGraphNode[]; edges: CaseGraphEdge[] }
export type AnalyzeMessageResponse = AnalysisResponse;
