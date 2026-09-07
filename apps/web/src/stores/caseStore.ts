import { createSignal } from 'solid-js';
import type { CaseSummary } from '@paugeran/shared';
import { listCases, createCase, updateCaseMode } from '../lib/api';

export const [cases, setCases] = createSignal<CaseSummary[]>([]);
export const [currentCase, setCurrentCase] = createSignal<CaseSummary | null>(null);
export const [isLoading, setIsLoading] = createSignal(false);
export const [error, setError] = createSignal<string | null>(null);

export async function fetchCases() {
  setIsLoading(true);
  setError(null);
  try {
    const data = await listCases();
    setCases(data);
  } catch (e) {
    setError(e instanceof Error ? e.message : 'Failed to load cases');
  } finally {
    setIsLoading(false);
  }
}

export async function createNewCase(title: string) {
  setIsLoading(true);
  setError(null);
  try {
    const newCase = await createCase(title);
    setCases([newCase, ...cases()]);
    return newCase;
  } catch (e) {
    setError(e instanceof Error ? e.message : 'Failed to create case');
    throw e;
  } finally {
    setIsLoading(false);
  }
}

export async function changeCaseMode(caseId: string, mode: import('@paugeran/shared').ReasoningMode) {
  try {
    const updated = await updateCaseMode(caseId, mode);
    setCases(cases().map((c) => (c.id === caseId ? updated : c)));
    if (currentCase()?.id === caseId) {
      setCurrentCase(updated);
    }
  } catch (e) {
    setError(e instanceof Error ? e.message : 'Failed to update mode');
  }
}
