/**
 * KnowledgeBaseSearch — Pencarian basis pengetahuan
 * [CB §24] — Supreme Legal Knowledge Base
 */

import { Component, createSignal } from 'solid-js';

interface KnowledgeBaseSearchProps {
  onSearch: (query: string) => void;
  onResultSelect: (id: string) => void;
}

const KnowledgeBaseSearch: Component<KnowledgeBaseSearchProps> = (props) => {
  const [query, setQuery] = createSignal('');

  const handleSearch = () => {
    if (query().trim()) {
      props.onSearch(query().trim());
    }
  };

  return (
    <div class="knowledge-base-search">
      <div class="flex gap-2">
        <input
          type="text"
          placeholder="Cari pasal, peraturan, atau yurisprudensi..."
          class="flex-1 px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800"
          value={query()}
          onInput={(e) => setQuery(e.currentTarget.value)}
          onKeyPress={(e) => e.key === 'Enter' && handleSearch()}
        />
        <button
          class="px-6 py-2 bg-primary-500 text-white rounded-lg hover:bg-primary-600 transition-colors"
          onClick={handleSearch}
        >
          Cari
        </button>
      </div>
    </div>
  );
};

export default KnowledgeBaseSearch;
