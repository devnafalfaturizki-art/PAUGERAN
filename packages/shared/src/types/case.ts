export type CaseState = 'unknown' | 'exploration' | 'preventive' | 'dispute' | 'litigation' | 'resolved';

export interface CaseSummary {
  id: string;
  title: string;
  state: CaseState;
  mode: import('./reasoning').ReasoningMode;
  updatedAt: string;
}
