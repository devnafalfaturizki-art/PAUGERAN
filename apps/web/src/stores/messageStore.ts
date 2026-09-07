import { createSignal } from 'solid-js';
import type { Message } from '@paugeran/shared';
import { analyzeMessage } from '../lib/api';

export const [messages, setMessages] = createSignal<Message[]>([]);
export const [isStreaming, setIsStreaming] = createSignal(false);

export async function sendMessage(caseId: string, content: string) {
  const userMessage: Message = {
    role: 'user',
    content,
    createdAt: new Date().toISOString(),
  };
  setMessages([...messages(), userMessage]);
  setIsStreaming(true);

  try {
    const response = await analyzeMessage(caseId, content);
    const systemMessage: Message = {
      role: 'system',
      content: response.content,
      certaintyScore: response.certaintyScore,
      createdAt: new Date().toISOString(),
    };
    setMessages([...messages(), systemMessage]);
  } catch (e) {
    console.error('Failed to send message:', e);
  } finally {
    setIsStreaming(false);
  }
}
