/**
 * KnowledgeBaseList — Daftar entri basis pengetahuan
 * [CB §24] — Supreme Legal Knowledge Base
 */

import { Component, createSignal } from 'solid-js';
import KnowledgeBaseDetail from './KnowledgeBaseDetail';
import KnowledgeBaseImport from './KnowledgeBaseImport';
import KnowledgeBaseSearch from './KnowledgeBaseSearch';

interface KnowledgeEntry {
  id: string;
  title: string;
  fullText: string;
  hierarchyLevel: number;
  tags: string[];
  sourceUrl?: string;
  createdAt: string;
}

interface KnowledgeBaseListProps {
  entries: KnowledgeEntry[];
  onAdd: () => void;
  onEdit: (id: string) => void;
  onDelete: (id: string) => void;
  onSearch: (query: string) => void;
  onImport: (file: File) => void;
}

const KnowledgeBaseList: Component<KnowledgeBaseListProps> = (props) => {
  const [selectedId, setSelectedId] = createSignal<string | null>(null);
  const [showImport, setShowImport] = createSignal(false);

  const selectedEntry = () => {
    if (!selectedId()) return null;
    return props.entries.find((e) => e.id === selectedId()) || null;
  };

  return (
    <div class="knowledge-base-list">
      <div class="flex items-center justify-between mb-4">
        <h3 class="text-lg font-semibold">Entri Basis Pengetahuan</h3>
        <div class="flex gap-2">
          <button
            class="px-4 py-2 bg-primary-500 text-white rounded-lg hover:bg-primary-600 transition-colors"
            onClick={() => props.onAdd()}
          >
            Tambah Entri
          </button>
          <button
            class="px-4 py-2 border border-gray-300 rounded-lg hover:bg-gray-50"
            onClick={() => setShowImport(true)}
          >
            Import
          </button>
        </div>
      </div>

      <div class="mb-4">
        <KnowledgeBaseSearch onSearch={props.onSearch} onResultSelect={setSelectedId} />
      </div>

      {showImport() && (
        <div class="mb-4">
          <KnowledgeBaseImport
            onImport={(file) => {
              props.onImport(file);
              setShowImport(false);
            }}
            onCancel={() => setShowImport(false)}
          />
        </div>
      )}

      <div class="grid grid-cols-1 lg:grid-cols-2 gap-4">
        <div class="space-y-3">
          {props.entries.length === 0 ? (
            <div class="text-center py-8 text-gray-500 dark:text-gray-400">
              Belum ada entri dalam basis pengetahuan.
            </div>
          ) : (
            props.entries.map((entry) => (
              <div
                class={`border rounded-lg p-4 cursor-pointer transition-colors ${
                  selectedId() === entry.id
                    ? 'border-primary-500 bg-primary-50 dark:bg-primary-900/20'
                    : 'border-gray-200 dark:border-gray-700 hover:border-gray-300'
                }`}
                onClick={() => setSelectedId(entry.id)}
              >
                <div class="font-medium">{entry.title}</div>
                <div class="text-sm text-gray-500 dark:text-gray-400">
                  Level {entry.hierarchyLevel} · {entry.tags.slice(0, 3).join(', ')}
                </div>
              </div>
            ))
          )}
        </div>

        <div>
          {selectedEntry() ? (
            <KnowledgeBaseDetail
              entry={selectedEntry()!}
              onEdit={() => props.onEdit(selectedEntry()!.id)}
              onDelete={() => props.onDelete(selectedEntry()!.id)}
            />
          ) : (
            <div class="text-center py-8 text-gray-500 dark:text-gray-400">
              Pilih entri untuk melihat detail
            </div>
          )}
        </div>
      </div>
    </div>
  );
};

export default KnowledgeBaseList;
