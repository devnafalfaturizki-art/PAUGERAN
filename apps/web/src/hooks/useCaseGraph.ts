import { createSignal } from 'solid-js';

export function useCaseGraph() {
  const [nodes, setNodes] = createSignal<import('@paugeran/shared').CaseGraphNode[]>([]);
  const [edges, setEdges] = createSignal<import('@paugeran/shared').CaseGraphEdge[]>([]);
  const [selectedNode, setSelectedNode] = createSignal<import('@paugeran/shared').CaseGraphNode | null>(null);

  return {
    get nodes() { return nodes(); },
    get edges() { return edges(); },
    get selectedNode() { return selectedNode(); },
    setNodes,
    setEdges,
    selectNode: setSelectedNode,
  };
}
