/**
 * PrecedentWeight — Indikator bobot preseden
 * [CB §47] — Precedential Weight System
 */

import { Component } from 'solid-js';

interface PrecedentWeightProps {
  weight: number;
  source: string;
  court: string;
  year: number;
}

const PrecedentWeight: Component<PrecedentWeightProps> = (props) => {
  const getWeightLabel = (weight: number) => {
    if (weight >= 0.9) return 'Sangat Tinggi';
    if (weight >= 0.7) return 'Tinggi';
    if (weight >= 0.5) return 'Sedang';
    if (weight >= 0.3) return 'Rendah';
    return 'Sangat Rendah';
  };

  const getWeightColor = (weight: number) => {
    if (weight >= 0.9) return 'text-green-600';
    if (weight >= 0.7) return 'text-blue-600';
    if (weight >= 0.5) return 'text-yellow-600';
    if (weight >= 0.3) return 'text-orange-600';
    return 'text-red-600';
  };

  return (
    <div class="precedent-weight">
      <div class="flex items-center justify-between">
        <div>
          <div class="text-sm font-medium">{props.source}</div>
          <div class="text-xs text-gray-500">
            {props.court} · {props.year}
          </div>
        </div>
        <div class={`text-sm font-semibold ${getWeightColor(props.weight)}`}>
          {getWeightLabel(props.weight)}
        </div>
      </div>
      <div class="mt-2 h-2 bg-gray-200 dark:bg-gray-700 rounded-full overflow-hidden">
        <div
          class={`h-full rounded-full ${
            props.weight >= 0.7
              ? 'bg-green-500'
              : props.weight >= 0.5
              ? 'bg-yellow-500'
              : 'bg-red-500'
          }`}
          style={{ width: `${props.weight * 100}%` }}
        />
      </div>
    </div>
  );
};

export default PrecedentWeight;
