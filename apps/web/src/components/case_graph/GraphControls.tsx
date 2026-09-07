/**
 * GraphControls — Kontrol navigasi Case Graph
 * [CB §22] — Case Graph Visualization
 */

import { Component } from 'solid-js';

interface GraphControlsProps {
  onZoomIn: () => void;
  onZoomOut: () => void;
  onReset: () => void;
  onFitView: () => void;
}

const GraphControls: Component<GraphControlsProps> = (props) => {
  return (
    <div class="graph-controls flex flex-col gap-2">
      <button
        class="p-2 bg-white dark:bg-gray-700 border border-gray-200 dark:border-gray-600 rounded shadow hover:bg-gray-50"
        onClick={props.onZoomIn}
        title="Zoom In"
      >
        +
      </button>
      <button
        class="p-2 bg-white dark:bg-gray-700 border border-gray-200 dark:border-gray-600 rounded shadow hover:bg-gray-50"
        onClick={props.onZoomOut}
        title="Zoom Out"
      >
        −
      </button>
      <button
        class="p-2 bg-white dark:bg-gray-700 border border-gray-200 dark:border-gray-600 rounded shadow hover:bg-gray-50"
        onClick={props.onFitView}
        title="Fit View"
      >
        ⛶
      </button>
      <button
        class="p-2 bg-white dark:bg-gray-700 border border-gray-200 dark:border-gray-600 rounded shadow hover:bg-gray-50"
        onClick={props.onReset}
        title="Reset"
      >
        ↺
      </button>
    </div>
  );
};

export default GraphControls;
