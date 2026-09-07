import { createSignal } from 'solid-js';

export function useMessages(_caseId: string) {
  const [_messages] = createSignal<import('@paugeran/shared').Message[]>([]);
  const [isStreaming, setIsStreaming] = createSignal(false);

  const send = async (_content: string) => {
    setIsStreaming(true);
    try {
      // Message sending logic
    } finally {
      setIsStreaming(false);
    }
  };

  return {
    get messages() { return _messages(); },
    get isStreaming() { return isStreaming(); },
    send,
  };
}
