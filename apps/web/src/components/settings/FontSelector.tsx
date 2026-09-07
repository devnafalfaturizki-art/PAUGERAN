/**
 * FontSelector — Pengaturan ukuran font
 * [CB §28] — Kustomisasi Antarmuka
 */

import { Component } from 'solid-js';

interface FontSelectorProps {
  fontSize: number;
  onFontSizeChange: (size: number) => void;
}

const FontSelector: Component<FontSelectorProps> = (props) => {
  const sizes = [
    { value: 12, label: 'Kecil' },
    { value: 14, label: 'Sedang' },
    { value: 16, label: 'Besar' },
    { value: 18, label: 'Sangat Besar' },
  ];

  return (
    <div class="font-selector">
      <h3 class="text-lg font-semibold mb-3">Ukuran Font</h3>
      <div class="flex gap-3">
        {sizes.map((size) => (
          <button
            class={`px-4 py-2 border-2 rounded-lg transition-colors ${
              props.fontSize === size.value
                ? 'border-primary-500 bg-primary-50 dark:bg-primary-900/20'
                : 'border-gray-200 dark:border-gray-700 hover:border-gray-300'
            }`}
            onClick={() => props.onFontSizeChange(size.value)}
          >
            {size.label}
          </button>
        ))}
      </div>
      <div class="mt-3 text-sm text-gray-500 dark:text-gray-400">
        Ukuran saat ini: {props.fontSize}px
      </div>
    </div>
  );
};

export default FontSelector;
