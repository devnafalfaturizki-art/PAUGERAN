import type { CaseGraphEdge, CaseGraphNode } from '../../../../../packages/shared/src';

interface CaseGraphViewerProps {
	nodes: CaseGraphNode[];
	edges: CaseGraphEdge[];
}

export default function CaseGraphViewer(props: CaseGraphViewerProps) {
	return (
		<section class="graph-card" aria-label="Case graph">
			<div class="section-title"><h3>Struktur perkara</h3><span>{props.nodes.length} node</span></div>
			<div class="next-steps">
				{props.nodes.map((node) => {
					const links = props.edges.filter((edge) => edge.sourceNodeId === node.id || edge.targetNodeId === node.id).length;
					return <div><strong>{node.nodeType}</strong><span> {node.content} · {links} relasi</span></div>;
				})}
				{props.nodes.length === 0 && <p class="muted">Belum ada node perkara.</p>}
			</div>
		</section>
	);
}
