interface StateBadgeProps { state: string }

export default function StateBadge(props: StateBadgeProps) {
	return <span class="state-chip">State: <strong>{props.state}</strong></span>;
}
