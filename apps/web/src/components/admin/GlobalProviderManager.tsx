/**
 * GlobalProviderManager — Manajemen penyedia LLM global
 * [CB §27] — Admin global providers
 */

import { Component } from 'solid-js';

interface GlobalProviderManagerProps {
  providers: Array<{
    id: string;
    name: string;
    providerType: string;
    model: string;
  }>;
  onAdd: () => void;
  onEdit: (id: string) => void;
  onDelete: (id: string) => void;
}

const GlobalProviderManager: Component<GlobalProviderManagerProps> = (props) => {
  return (
    <div class="global-provider-manager">
      <div class="flex items-center justify-between mb-4">
        <h3 class="text-lg font-semibold">Penyedia LLM Global</h3>
        <button
          class="px-4 py-2 bg-primary-500 text-white rounded-lg hover:bg-primary-600 transition-colors"
          onClick={props.onAdd}
        >
          Tambah Penyedia
        </button>
      </div>

      {props.providers.length === 0 ? (
        <div class="text-center py-8 text-gray-500 dark:text-gray-400">
          Belum ada penyedia global. Tambahkan untuk memberikannya ke seluruh tim.
        </div>
      ) : (
        <div class="space-y-3">
          {props.providers.map((provider) => (
            <div class="flex items-center justify-between border border-gray-200 dark:border-gray-700 rounded-lg p-4">
              <div>
                <div class="font-medium">{provider.name}</div>
                <div class="text-sm text-gray-500 dark:text-gray-400">
                  {provider.providerType} — {provider.model}
                </div>
              </div>
              <div class="flex gap-2">
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
          ))}
        </div>
      )}
    </div>
  );
};

export default GlobalProviderManager;
