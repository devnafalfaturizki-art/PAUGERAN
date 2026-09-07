/**
 * DataManager — Manajemen data dan backup
 * [CB §3.7] — Data & Ekspor
 */

import { Component } from 'solid-js';

interface DataManagerProps {
  onExport: () => void;
  onImport: (file: File) => void;
  onBackup: () => void;
  onRestore: () => void;
}

const DataManager: Component<DataManagerProps> = (props) => {
  return (
    <div class="data-manager">
      <h3 class="text-lg font-semibold mb-3">Data & Backup</h3>
      
      <div class="space-y-4">
        <div class="border-2 border-dashed border-gray-300 dark:border-gray-600 rounded-lg p-6 text-center">
          <div class="text-4xl mb-2">📦</div>
          <div class="font-medium mb-1">Backup Data</div>
          <div class="text-sm text-gray-500 dark:text-gray-400 mb-3">
            Ekspor seluruh data kasus dan pengaturan
          </div>
          <button
            class="px-4 py-2 bg-primary-500 text-white rounded-lg hover:bg-primary-600 transition-colors"
            onClick={props.onBackup}
          >
            Backup Sekarang
          </button>
        </div>

        <div class="border-2 border-dashed border-gray-300 dark:border-gray-600 rounded-lg p-6 text-center">
          <div class="text-4xl mb-2">📥</div>
          <div class="font-medium mb-1">Restore Data</div>
          <div class="text-sm text-gray-500 dark:text-gray-400 mb-3">
            Pulihkan data dari file backup
          </div>
          <label class="inline-block px-4 py-2 bg-gray-500 text-white rounded-lg hover:bg-gray-600 transition-colors cursor-pointer">
            Pilih File Backup
            <input
              type="file"
              accept=".json,.db"
              class="hidden"
              onChange={(e) => {
                const file = e.currentTarget.files?.[0];
                if (file) props.onImport(file);
              }}
            />
          </label>
        </div>

        <div class="border-2 border-dashed border-gray-300 dark:border-gray-600 rounded-lg p-6 text-center">
          <div class="text-4xl mb-2">📄</div>
          <div class="font-medium mb-1">Ekspor Laporan</div>
          <div class="text-sm text-gray-500 dark:text-gray-400 mb-3">
            Ekspor laporan kasus dalam format PDF atau DOCX
          </div>
          <button
            class="px-4 py-2 bg-primary-500 text-white rounded-lg hover:bg-primary-600 transition-colors"
            onClick={props.onExport}
          >
            Ekspor Laporan
          </button>
        </div>
      </div>
    </div>
  );
};

export default DataManager;
