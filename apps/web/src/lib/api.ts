import type { CaseSummary, ReasoningMode } from '../../../../packages/shared/src';

export type AnalysisResponse = {
  role: 'system';
  content: string;
  certaintyScore: number;
  clarifyingQuestions: string[];
};

async function request<T>(path: string, init?: RequestInit): Promise<T> {
  const response = await fetch(path, init);
  if (!response.ok) {
    throw new Error(`API request failed: ${response.status}`);
  }
  return (await response.json()) as T;
}

export function listCases(): Promise<CaseSummary[]> {
  return request<CaseSummary[]>('/api/cases');
}

export function createCase(title: string): Promise<CaseSummary> {
  return request<CaseSummary>('/api/cases', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ title }),
  });
}

export function updateCaseMode(caseId: string, mode: ReasoningMode): Promise<CaseSummary> {
  return request<CaseSummary>(`/api/cases/${caseId}/mode`, {
    method: 'PATCH',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ mode }),
  });
}

export function analyzeMessage(caseId: string, content: string): Promise<AnalysisResponse> {
  return request<AnalysisResponse>(`/api/cases/${caseId}/messages`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ content }),
  });
}

export function streamAnalysis(caseId: string, onEvent: (event: MessageEvent) => void): EventSource {
  const source = new EventSource(`/api/cases/${caseId}/messages/stream`);
  source.addEventListener('phase', onEvent);
  source.addEventListener('complete', onEvent);
  return source;
}
