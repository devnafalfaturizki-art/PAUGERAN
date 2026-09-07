/**
 * NodeDetail — Panel detail node pada Case Graph
 * [CB §22] — Case Graph & Knowledge Representation
 */

import { Component } from 'solid-js';
import type { CaseGraphNode } from '@paugeran/shared';

interface NodeDetailProps {
  node: CaseGraphNode | null;
  onClose: () => void;
}

const NodeDetail: Component<NodeDetailProps> = (props) => {
  if (!props.node) {
    return (
      <div class="node-detail empty">
        <p class="text-gray-500 dark:text-gray-400 text-center">
          Pilih node untuk melihat detail
        </p>
      </div>
    );
  }

  return (
    <div class="node-detail">
      <div class="flex items-center justify-between mb-4">
        <h3 class="text-lg font-semibold capitalize">{props.node.nodeType}</h3>
        <button
          class="text-gray-500 hover:text-gray-700"
          onClick={props.onClose}
        >
          ✕
        </button>
      </div>

      <div class="mb-4">
        <div class="text-sm font-medium mb-1">Konten</div>
        <div class="p-3 bg-gray-50 dark:bg-gray-700 rounded-lg text-sm">
          {props.node.content}
        </div>
      </div>

      <div class="mb-4">
        <div class="text-sm font-medium mb-1">Metadata</div>
        <pre class="p-3 bg-gray-50 dark:bg-gray-700 rounded-lg text-xs overflow-auto">
          {JSON.stringify(props.node.metadata, null, 2)}
        </pre>
      </div>

      <div class="text-xs text-gray-500">
        Dibuat: {new Date(props.node.createdAt).toLocaleString('id-ID')}
      </div>
    </div>
  );
};

export default NodeDetail;
