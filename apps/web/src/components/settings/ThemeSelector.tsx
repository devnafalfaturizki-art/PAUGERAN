/**
 * ThemeSelector — Tema warna antarmuka
 * [CB §28] — Kustomisasi Antarmuka
 */

import { Component } from 'solid-js';

interface ThemeSelectorProps {
  currentTheme: string;
  onThemeChange: (theme: string) => void;
}

const ThemeSelector: Component<ThemeSelectorProps> = (props) => {
  const themes = [
    { id: 'light', label: 'Light', description: 'Tema terang' },
    { id: 'dark', label: 'Dark', description: 'Tema gelap' },
    { id: 'sepia', label: 'Sepia', description: 'Tema hangat' },
    { id: 'high-contrast', label: 'High Contrast', description: 'Kontras tinggi untuk aksesibilitas' },
  ];

  return (
    <div class="theme-selector">
      <h3 class="text-lg font-semibold mb-3">Tema</h3>
      <div class="grid grid-cols-2 gap-3">
        {themes.map((theme) => (
          <button
            class={`p-3 border-2 rounded-lg text-left transition-colors ${
              props.currentTheme === theme.id
                ? 'border-primary-500 bg-primary-50 dark:bg-primary-900/20'
                : 'border-gray-200 dark:border-gray-700 hover:border-gray-300'
            }`}
            onClick={() => props.onThemeChange(theme.id)}
          >
            <div class="font-medium">{theme.label}</div>
            <div class="text-sm text-gray-500 dark:text-gray-400">{theme.description}</div>
          </button>
        ))}
      </div>
    </div>
  );
};

export default ThemeSelector;
