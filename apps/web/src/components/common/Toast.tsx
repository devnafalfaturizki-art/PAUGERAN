interface ToastProps { message: string; kind?: 'info' | 'error' }

export default function Toast(props: ToastProps) {
	return <div class={`trace-note toast-${props.kind ?? 'info'}`} role="status">{props.message}</div>;
}
