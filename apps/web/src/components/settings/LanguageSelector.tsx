/**
 * LanguageSelector — Pemilihan bahasa
 * [CB §28] — Kustomisasi Antarmuka
 */

import { Component } from 'solid-js';

interface LanguageSelectorProps {
  language: string;
  onLanguageChange: (language: string) => void;
}

const LanguageSelector: Component<LanguageSelectorProps> = (props) => {
  const languages = [
    { id: 'id', label: 'Bahasa Indonesia', native: 'Indonesia' },
    { id: 'en', label: 'English', native: 'English' },
  ];

  return (
    <div class="language-selector">
      <h3 class="text-lg font-semibold mb-3">Bahasa</h3>
      <div class="space-y-2">
        {languages.map((lang) => (
          <button
            class={`w-full p-3 border-2 rounded-lg text-left transition-colors ${
              props.language === lang.id
                ? 'border-primary-500 bg-primary-50 dark:bg-primary-900/20'
                : 'border-gray-200 dark:border-gray-700 hover:border-gray-300'
            }`}
            onClick={() => props.onLanguageChange(lang.id)}
          >
            <div class="font-medium">{lang.label}</div>
            <div class="text-sm text-gray-500 dark:text-gray-400">{lang.native}</div>
          </button>
        ))}
      </div>
    </div>
  );
};

export default LanguageSelector;
