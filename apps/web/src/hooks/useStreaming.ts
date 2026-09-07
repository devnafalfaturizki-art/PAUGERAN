export function useStreaming(caseId: string, onEvent: (event: { type: string; data: unknown }) => void) {
  let source: EventSource | null = null;

  const start = () => {
    source = new EventSource(`/api/cases/${caseId}/messages/stream`);
    source.addEventListener('phase', (event: MessageEvent) => {
      onEvent({ type: 'phase', data: event.data });
    });
    source.addEventListener('complete', (event: MessageEvent) => {
      onEvent({ type: 'complete', data: event.data });
      source?.close();
    });
    source.onerror = () => {
      source?.close();
    };
  };

  const stop = () => {
    source?.close();
    source = null;
  };

  return { start, stop };
}
