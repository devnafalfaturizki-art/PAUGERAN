/**
 * AccessibilitySettings — Pengaturan aksesibilitas
 * [CB §34] — Aksesibilitas
 */

import { Component } from 'solid-js';

interface AccessibilitySettingsProps {
  highContrast: boolean;
  reducedMotion: boolean;
  screenReaderMode: boolean;
  onToggle: (setting: string, value: boolean) => void;
}

const AccessibilitySettings: Component<AccessibilitySettingsProps> = (props) => {
  return (
    <div class="accessibility-settings">
      <h3 class="text-lg font-semibold mb-3">Aksesibilitas</h3>
      
      <div class="space-y-4">
        <div class="flex items-center justify-between">
          <div>
            <div class="font-medium">Mode Kontras Tinggi</div>
            <div class="text-sm text-gray-500 dark:text-gray-400">
              Tingkatkan kontras untuk keterbacaan yang lebih baik
            </div>
          </div>
          <button
            class={`w-12 h-6 rounded-full transition-colors ${
              props.highContrast ? 'bg-primary-500' : 'bg-gray-300'
            }`}
            onClick={() => props.onToggle('highContrast', !props.highContrast)}
          >
            <div
              class={`w-4 h-4 bg-white rounded-full shadow transition-transform ${
                props.highContrast ? 'translate-x-7' : 'translate-x-1'
              }`}
            />
          </button>
        </div>

        <div class="flex items-center justify-between">
          <div>
            <div class="font-medium">Kurangi Gerakan</div>
            <div class="text-sm text-gray-500 dark:text-gray-400">
              Kurangi animasi dan transisi
            </div>
          </div>
          <button
            class={`w-12 h-6 rounded-full transition-colors ${
              props.reducedMotion ? 'bg-primary-500' : 'bg-gray-300'
            }`}
            onClick={() => props.onToggle('reducedMotion', !props.reducedMotion)}
          >
            <div
              class={`w-4 h-4 bg-white rounded-full shadow transition-transform ${
                props.reducedMotion ? 'translate-x-7' : 'translate-x-1'
              }`}
            />
          </button>
        </div>

        <div class="flex items-center justify-between">
          <div>
            <div class="font-medium">Mode Layar Pembaca</div>
            <div class="text-sm text-gray-500 dark:text-gray-400">
              Optimalkan untuk screen reader
            </div>
          </div>
          <button
            class={`w-12 h-6 rounded-full transition-colors ${
              props.screenReaderMode ? 'bg-primary-500' : 'bg-gray-300'
            }`}
            onClick={() => props.onToggle('screenReaderMode', !props.screenReaderMode)}
          >
            <div
              class={`w-4 h-4 bg-white rounded-full shadow transition-transform ${
                props.screenReaderMode ? 'translate-x-7' : 'translate-x-1'
              }`}
            />
          </button>
        </div>
      </div>
    </div>
  );
};

export default AccessibilitySettings;
