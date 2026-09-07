/**
 * LLMProviderManager — Manajemen penyedia LLM
 * [CB §26] — Multi-Provider LLM
 */

import { Component } from 'solid-js';

interface LLMProviderManagerProps {
  providers: Array<{
    id: string;
    name: string;
    providerType: string;
    model: string;
    isGlobal?: boolean;
  }>;
  onAdd: () => void;
  onEdit: (id: string) => void;
  onDelete: (id: string) => void;
  onTest: (id: string) => void;
}

const LLMProviderManager: Component<LLMProviderManagerProps> = (props) => {
  return (
    <div class="llm-provider-manager">
      <div class="flex items-center justify-between mb-4">
        <h3 class="text-lg font-semibold">Penyedia LLM</h3>
        <button
          class="px-4 py-2 bg-primary-500 text-white rounded-lg hover:bg-primary-600 transition-colors"
          onClick={props.onAdd}
        >
          Tambah Penyedia
        </button>
      </div>

      {props.providers.length === 0 ? (
        <div class="text-center py-8 text-gray-500 dark:text-gray-400">
          Belum ada penyedia LLM yang dikonfigurasi.
          <br />
          Tambahkan penyedia untuk memulai.
        </div>
      ) : (
        <div class="space-y-3">
          {props.providers.map((provider) => (
            <div class="border border-gray-200 dark:border-gray-700 rounded-lg p-4">
              <div class="flex items-center justify-between">
                <div>
                  <div class="font-medium">{provider.name}</div>
                  <div class="text-sm text-gray-500 dark:text-gray-400">
                    {provider.providerType} — {provider.model}
                  </div>
                  {provider.isGlobal && (
                    <span class="inline-block mt-1 px-2 py-0.5 text-xs bg-blue-100 text-blue-800 rounded">
                      Global
                    </span>
                  )}
                </div>
                <div class="flex gap-2">
                  <button
                    class="px-3 py-1 text-sm border border-gray-300 rounded hover:bg-gray-50"
                    onClick={() => props.onTest(provider.id)}
                  >
                    Test
                  </button>
                  <button
                    class="px-3 py-1 text-sm border border-gray-300 rounded hover:bg-gray-50"
                    onClick={() => props.onEdit(provider.id)}
                  >
                    Edit
                  </button>
                  <button
                    class="px-3 py-1 text-sm border border-red-300 text-red-600 rounded hover:bg-red-50"
                    onClick={() => props.onDelete(provider.id)}
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

export default LLMProviderManager;
