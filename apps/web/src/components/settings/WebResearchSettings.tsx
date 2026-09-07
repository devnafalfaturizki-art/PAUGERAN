/**
 * WebResearchSettings — Pengaturan penelitian web
 * [CB §25] — Penelitian Web
 */

import { Component } from 'solid-js';

interface WebResearchSettingsProps {
  enabled: boolean;
  maxDepth: number;
  onToggle: (enabled: boolean) => void;
  onMaxDepthChange: (depth: number) => void;
}

const WebResearchSettings: Component<WebResearchSettingsProps> = (props) => {
  return (
    <div class="web-research-settings">
      <h3 class="text-lg font-semibold mb-3">Penelitian Web</h3>
      
      <div class="space-y-4">
        <div class="flex items-center justify-between">
          <div>
            <div class="font-medium">Aktifkan Penelitian Web</div>
            <div class="text-sm text-gray-500 dark:text-gray-400">
              Izinkan PAUGERAN melakukan penelitian hukum dari internet
            </div>
          </div>
          <button
            class={`w-12 h-6 rounded-full transition-colors ${
              props.enabled ? 'bg-primary-500' : 'bg-gray-300'
            }`}
            onClick={() => props.onToggle(!props.enabled)}
          >
            <div
              class={`w-4 h-4 bg-white rounded-full shadow transition-transform ${
                props.enabled ? 'translate-x-7' : 'translate-x-1'
              }`}
            />
          </button>
        </div>

        <div>
          <div class="font-medium mb-2">Kedalaman Penelitian</div>
          <div class="flex items-center gap-4">
            <input
              type="range"
              min="1"
              max="5"
              value={props.maxDepth}
              onInput={(e) => props.onMaxDepthChange(Number(e.currentTarget.value))}
              class="flex-1"
            />
            <span class="text-sm text-gray-500">{props.maxDepth} level</span>
          </div>
          <div class="text-xs text-gray-400 mt-1">
            Level 1: Sumber resmi saja | Level 5: Semua sumber tepercaya
          </div>
        </div>
      </div>
    </div>
  );
};

export default WebResearchSettings;
