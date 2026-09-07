/**
 * GraphLegend — Legenda untuk Case Graph
 * [CB §22] — Case Graph Visualization
 */

import { Component } from 'solid-js';
import { NODE_TYPES } from '@paugeran/shared';

const GraphLegend: Component = () => {
  const colors: Record<string, string> = {
    case: 'bg-blue-500',
    party: 'bg-green-500',
    fact: 'bg-yellow-500',
    evidence: 'bg-purple-500',
    issue: 'bg-red-500',
    rule: 'bg-indigo-500',
    source: 'bg-gray-500',
    argument: 'bg-teal-500',
    counterargument: 'bg-orange-500',
    risk: 'bg-pink-500',
    conclusion: 'bg-emerald-500',
    document: 'bg-cyan-500',
  };

  return (
    <div class="graph-legend">
      <h4 class="text-sm font-semibold mb-2">Legenda</h4>
      <div class="space-y-1">
        {NODE_TYPES.map((type) => (
          <div class="flex items-center gap-2 text-xs">
            <span class={`w-3 h-3 rounded-full ${colors[type] || 'bg-gray-400'}`} />
            <span class="capitalize">{type.replace(/_/g, ' ')}</span>
          </div>
        ))}
      </div>
    </div>
  );
};

export default GraphLegend;
