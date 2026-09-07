export type ReasoningMode = 'exploration' | 'preventive' | 'dispute' | 'litigation_prep' | 'adversarial' | 'neutral';

export interface AnalysisResponse {
  role: 'system';
  content: string;
  mode: ReasoningMode;
  certaintyScore: number;
  factors: string[];
  clarifyingQuestions: string[];
}
