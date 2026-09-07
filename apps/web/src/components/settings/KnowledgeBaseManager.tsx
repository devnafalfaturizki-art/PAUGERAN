/**
 * KnowledgeBaseManager — Manajemen basis pengetahuan hukum
 * [CB §24] — Supreme Legal Knowledge Base
 */

import { Component } from 'solid-js';

interface KnowledgeBaseManagerProps {
  entries: Array<{
    id: string;
    title: string;
    tags: string[];
  }>;
  onAdd: () => void;
  onEdit: (id: string) => void;
  onDelete: (id: string) => void;
  onSearch: (query: string) => void;
}

const KnowledgeBaseManager: Component<KnowledgeBaseManagerProps> = (props) => {
  return (
    <div class="knowledge-base-manager">
      <div class="flex items-center justify-between mb-4">
        <h3 class="text-lg font-semibold">Basis Pengetahuan Hukum</h3>
        <button
          class="px-4 py-2 bg-primary-500 text-white rounded-lg hover:bg-primary-600 transition-colors"
          onClick={props.onAdd}
        >
          Tambah Entri
        </button>
      </div>

      <div class="mb-4">
        <input
          type="text"
          placeholder="Cari dalam basis pengetahuan..."
          class="w-full px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800"
          onInput={(e) => props.onSearch(e.currentTarget.value)}
        />
      </div>

      {props.entries.length === 0 ? (
        <div class="text-center py-8 text-gray-500 dark:text-gray-400">
          Basis pengetahuan hukum masih kosong.
          <br />
          Tambahkan peraturan, pasal, atau putusan untuk mulai.
        </div>
      ) : (
        <div class="space-y-3">
          {props.entries.map((entry) => (
            <div class="border border-gray-200 dark:border-gray-700 rounded-lg p-4">
              <div class="flex items-center justify-between">
                <div>
                  <div class="font-medium">{entry.title}</div>
                  <div class="text-sm text-gray-500 dark:text-gray-400">
                    {entry.tags.join(', ')}
                  </div>
                </div>
                <div class="flex gap-2">
                  <button
                    class="px-3 py-1 text-sm border border-gray-300 rounded hover:bg-gray-50"
                    onClick={() => props.onEdit(entry.id)}
                  >
                    Edit
                  </button>
                  <button
                    class="px-3 py-1 text-sm border border-red-300 text-red-600 rounded hover:bg-red-50"
                    onClick={() => props.onDelete(entry.id)}
                  >
                    Hapus
                  </button>
                </div>
              </div>
            </div>
          ))}
        </div>
      )}
    </div>
  );
};

export default KnowledgeBaseManager;
