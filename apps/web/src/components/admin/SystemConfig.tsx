/**
 * SystemConfig — Konfigurasi sistem
 * [CB §27] — Admin system configuration
 */

import { Component, createSignal } from 'solid-js';

interface SystemConfigProps {
  config: {
    appName: string;
    version: string;
    authEnabled: boolean;
    maxCasesPerUser: number;
    maxDocumentSize: number;
  };
  onSave: (config: Record<string, unknown>) => void;
}

const SystemConfig: Component<SystemConfigProps> = (props) => {
  const [authEnabled, setAuthEnabled] = createSignal(props.config.authEnabled);
  const [maxCases, setMaxCases] = createSignal(props.config.maxCasesPerUser);
  const [maxDocSize, setMaxDocSize] = createSignal(props.config.maxDocumentSize);

  const handleSave = () => {
    props.onSave({
      authEnabled: authEnabled(),
      maxCasesPerUser: maxCases(),
      maxDocumentSize: maxDocSize(),
    });
  };

  return (
    <div class="system-config">
      <h3 class="text-lg font-semibold mb-4">Konfigurasi Sistem</h3>

      <div class="space-y-6">
        <div class="border-b border-gray-200 dark:border-gray-700 pb-4">
          <div class="font-medium mb-2">Informasi Aplikasi</div>
          <div class="text-sm text-gray-500 dark:text-gray-400">
            <div>Nama: {props.config.appName}</div>
            <div>Versi: {props.config.version}</div>
          </div>
        </div>

        <div class="flex items-center justify-between">
          <div>
            <div class="font-medium">Autentikasi Multi-User</div>
            <div class="text-sm text-gray-500 dark:text-gray-400">
              Aktifkan sistem autentikasi untuk tim
            </div>
          </div>
          <button
            class={`w-12 h-6 rounded-full transition-colors ${
              authEnabled() ? 'bg-primary-500' : 'bg-gray-300'
            }`}
            onClick={() => setAuthEnabled(!authEnabled())}
          >
            <div
              class={`w-4 h-4 bg-white rounded-full shadow transition-transform ${
                authEnabled() ? 'translate-x-7' : 'translate-x-1'
              }`}
            />
          </button>
        </div>

        <div>
          <div class="font-medium mb-2">Batas Kasus per Pengguna</div>
          <input
            type="number"
            class="w-full px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800"
            value={maxCases()}
            onInput={(e) => setMaxCases(Number(e.currentTarget.value))}
          />
        </div>

        <div>
          <div class="font-medium mb-2">Ukuran Maksimal Dokumen (MB)</div>
          <input
            type="number"
            class="w-full px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800"
            value={maxDocSize()}
            onInput={(e) => setMaxDocSize(Number(e.currentTarget.value))}
          />
        </div>

        <div class="pt-4">
          <button
            class="px-6 py-2 bg-primary-500 text-white rounded-lg hover:bg-primary-600 transition-colors"
            onClick={handleSave}
          >
            Simpan Konfigurasi
          </button>
        </div>
      </div>
    </div>
  );
};

export default SystemConfig;
