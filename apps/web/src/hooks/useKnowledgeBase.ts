import { createSignal } from 'solid-js';
import type { KnowledgeEntry } from '@paugeran/shared';
import { listKnowledge, addKnowledge } from '../lib/api';

export function useKnowledgeBase() {
  const [entries, setEntries] = createSignal<KnowledgeEntry[]>([]);
  const [isLoading, setIsLoading] = createSignal(false);
  const [error, setError] = createSignal<string | null>(null);

  const fetchEntries = async () => {
    setIsLoading(true);
    setError(null);
    try {
      const data = await listKnowledge();
      setEntries(data);
    } catch (e) {
      setError(e instanceof Error ? e.message : 'Failed to load knowledge base');
    } finally {
      setIsLoading(false);
    }
  };

  const addEntry = async (entry: Omit<KnowledgeEntry, 'id' | 'createdAt'>) => {
    setIsLoading(true);
    setError(null);
    try {
      const newEntry = await addKnowledge(entry);
      setEntries([...entries(), newEntry]);
    } catch (e) {
      setError(e instanceof Error ? e.message : 'Failed to add entry');
      throw e;
    } finally {
      setIsLoading(false);
    }
  };

  return {
    get entries() { return entries(); },
    get isLoading() { return isLoading(); },
    get error() { return error(); },
    fetchEntries,
    addEntry,
  };
}
