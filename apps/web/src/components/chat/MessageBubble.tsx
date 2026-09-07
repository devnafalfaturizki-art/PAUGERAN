interface MessageBubbleProps {
	role: 'system' | 'user';
	content: string;
}

export default function MessageBubble(props: MessageBubbleProps) {
	return (
		<article class={`message ${props.role}`}>
			<span class="message-label">{props.role === 'system' ? 'PAUGERAN' : 'ANDA'}</span>
			<p>{props.content}</p>
		</article>
	);
}
