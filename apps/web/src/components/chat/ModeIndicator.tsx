interface ModeIndicatorProps { mode: string; tone?: string }

export default function ModeIndicator(props: ModeIndicatorProps) {
	return <div class="mode-label"><span class={`mode-dot ${props.tone ?? 'slate'}`} /><span>Mode penalaran</span><strong>{props.mode}</strong></div>;
}
