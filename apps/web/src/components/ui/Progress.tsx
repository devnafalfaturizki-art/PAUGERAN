interface ProgressProps { value: number; label?: string }

export default function Progress(props: ProgressProps) {
	const value = Math.max(0, Math.min(100, props.value));
	return <div aria-label={props.label ?? 'Progress'} class="progress" role="progressbar" aria-valuenow={value} aria-valuemin="0" aria-valuemax="100"><span style={{ width: `${value}%` }} /></div>;
}
