/**
 * SSE client utilities for streaming analysis.
 */

export function createEventSource(url: string): EventSource {
  return new EventSource(url);
}

export function parseSSEEvent(event: MessageEvent): { type: string; data: unknown } {
  try {
    return {
      type: event.type || 'message',
      data: JSON.parse(event.data),
    };
  } catch {
    return {
      type: event.type || 'message',
      data: event.data,
    };
  }
}
