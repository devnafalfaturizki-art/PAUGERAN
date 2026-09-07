/**
 * GraphExport — Ekspor Case Graph sebagai PNG/SVG
 * [CB §22] — Case Graph Visualization
 */

import { Component } from 'solid-js';

interface GraphExportProps {
  onExportPng: () => void;
  onExportSvg: () => void;
}

const GraphExport: Component<GraphExportProps> = (props) => {
  return (
    <div class="graph-export">
      <h4 class="text-sm font-semibold mb-2">Ekspor Graph</h4>
      <div class="flex gap-2">
        <button
          class="px-3 py-1 text-sm border border-gray-300 rounded hover:bg-gray-50"
          onClick={props.onExportPng}
        >
          Export PNG
        </button>
        <button
          class="px-3 py-1 text-sm border border-gray-300 rounded hover:bg-gray-50"
          onClick={props.onExportSvg}
        >
          Export SVG
        </button>
      </div>
    </div>
  );
};

export default GraphExport;
