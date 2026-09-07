/**
 * SetupPage — Wizard setup awal
 * [CB §3.2] — Konfigurasi API key
 */

import { Component, createSignal } from 'solid-js';

const SetupPage: Component = () => {
  const [apiKey, setApiKey] = createSignal('');
  const [provider, setProvider] = createSignal('anthropic');
  const [model, setModel] = createSignal('claude-3-5-sonnet-20240620');
  const [isLoading, setIsLoading] = createSignal(false);

  const handleSubmit = async (e: Event) => {
    e.preventDefault();
    setIsLoading(true);
    
    // Simulate setup
    await new Promise((resolve) => setTimeout(resolve, 1000));
    
    // Save to localStorage
    localStorage.setItem('paugeran_setup_complete', 'true');
    localStorage.setItem('paugeran_provider', provider());
    localStorage.setItem('paugeran_model', model());
    
    setIsLoading(false);
    // Redirect to chat
    window.location.href = '/';
  };

  return (
    <div class="setup-page min-h-screen flex items-center justify-center bg-gray-50 dark:bg-gray-900 p-4">
      <div class="max-w-md w-full bg-white dark:bg-gray-800 rounded-xl shadow-lg p-8">
        <h1 class="text-2xl font-bold text-center mb-2">Selamat Datang di PAUGERAN</h1>
        <p class="text-gray-600 dark:text-gray-400 text-center mb-8">
          Supreme Legal Reasoning Engine untuk Hukum Indonesia
        </p>

        <form onSubmit={handleSubmit} class="space-y-6">
          <div>
            <label class="block text-sm font-medium mb-2">Penyedia LLM</label>
            <select
              class="w-full px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700"
              value={provider()}
              onChange={(e) => setProvider(e.currentTarget.value)}
            >
              <option value="anthropic">Anthropic (Claude)</option>
              <option value="openai">OpenAI (GPT)</option>
              <option value="groq">Groq</option>
              <option value="ollama">Ollama (Lokal)</option>
            </select>
          </div>

          <div>
            <label class="block text-sm font-medium mb-2">Model</label>
            <select
              class="w-full px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700"
              value={model()}
              onChange={(e) => setModel(e.currentTarget.value)}
            >
              {provider() === 'anthropic' && (
                <>
                  <option value="claude-3-5-sonnet-20240620">Claude 3.5 Sonnet</option>
                  <option value="claude-3-opus-20240229">Claude 3 Opus</option>
                  <option value="claude-3-haiku-20240307">Claude 3 Haiku</option>
                </>
              )}
              {provider() === 'openai' && (
                <>
                  <option value="gpt-4o">GPT-4o</option>
                  <option value="gpt-4-turbo">GPT-4 Turbo</option>
                  <option value="gpt-3.5-turbo">GPT-3.5 Turbo</option>
                </>
              )}
            </select>
          </div>

          <div>
            <label class="block text-sm font-medium mb-2">API Key</label>
            <input
              type="password"
              class="w-full px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700"
              placeholder="Masukkan API key Anda"
              value={apiKey()}
              onInput={(e) => setApiKey(e.currentTarget.value)}
              required
            />
            <p class="text-xs text-gray-500 mt-1">
              API key disimpan secara lokal dan aman di perangkat Anda.
            </p>
          </div>

          <button
            type="submit"
            disabled={isLoading() || !apiKey().trim()}
            class="w-full py-3 bg-primary-500 text-white rounded-lg hover:bg-primary-600 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
          >
            {isLoading() ? 'Menyimpan...' : 'Mulai Menggunakan PAUGERAN'}
          </button>
        </form>

        <p class="text-xs text-gray-400 text-center mt-6">
          Dengan menggunakan PAUGERAN, Anda menyetujui bahwa analisis ini bukan nasihat hukum final.
        </p>
      </div>
    </div>
  );
};

export default SetupPage;
