import { createSignal } from 'solid-js';
import type { CaseSummary } from '@paugeran/shared';

export function useCases() {
  const [_cases] = createSignal<CaseSummary[]>([]);
  const [isLoading, setIsLoading] = createSignal(false);

  const refreshCases = async () => {
    setIsLoading(true);
    try {
      // Cases logic
    } catch {
      // ignore
    } finally {
      setIsLoading(false);
    }
  };

  return {
    get cases() { return _cases(); },
    get isLoading() { return isLoading(); },
    refreshCases,
  };
}
