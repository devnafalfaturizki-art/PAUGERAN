/**
 * LayerDisplay — Tampilan 7 lapisan penalaran hukum
 * [CB §7] — Legal Reasoning Layers
 */

import { Component } from 'solid-js';
import type { LayerInterpretation } from '@paugeran/shared';

interface LayerDisplayProps {
  layers: LayerInterpretation[];
  synthesis?: string;
  finalCertainty?: number;
}

const LayerDisplay: Component<LayerDisplayProps> = (props) => {
  const layerIcons: Record<string, string> = {
    grammatical: '📝',
    systematic: '🏛️',
    teleological: '🎯',
    sociological: '🌍',
    historical: '📜',
    comparative: '⚖️',
    critical: '🔍',
  };

  return (
    <section aria-label="Tujuh lapisan penalaran">
      <div class="section-title">
        <h3>Lapisan Penalaran Hukum</h3>
        <span>07</span>
      </div>

      <div class="space-y-3">
        {props.layers.map((layer) => (
          <div class="border border-gray-200 dark:border-gray-700 rounded-lg p-4">
            <div class="flex items-center gap-2 mb-2">
              <span class="text-xl">{layerIcons[layer.layer] || '📄'}</span>
              <h4 class="font-semibold">{layer.title}</h4>
              <span class="ml-auto text-xs text-gray-500">
                Kepastian: {(layer.certainty * 100).toFixed(0)}%
              </span>
            </div>
            <p class="text-sm text-gray-700 dark:text-gray-300 mb-2">{layer.content}</p>
            {layer.sources.length > 0 && (
              <div class="text-xs text-gray-500">
                <strong>Sumber:</strong> {layer.sources.join(', ')}
              </div>
            )}
          </div>
        ))}
      </div>

      {props.synthesis && (
        <div class="mt-4 p-4 bg-primary-50 dark:bg-primary-900/20 rounded-lg">
          <h4 class="font-semibold mb-2">Sintesis</h4>
          <p class="text-sm">{props.synthesis}</p>
          {props.finalCertainty !== undefined && (
            <div class="mt-2 text-sm">
              <strong>Tingkat Kepastian:</strong> {(props.finalCertainty * 100).toFixed(0)}%
            </div>
          )}
        </div>
      )}
    </section>
  );
};

export default LayerDisplay;
