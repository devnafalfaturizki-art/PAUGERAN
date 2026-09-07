export interface Provider {
  id?: string;
  name: string;
  providerType: 'anthropic' | 'openai' | 'groq' | 'together' | 'fireworks' | 'openrouter' | 'mistral' | 'deepseek' | 'ollama' | 'lmstudio' | 'vllm' | 'custom';
  apiKey: string;
  baseUrl?: string;
  model: string;
  isGlobal?: boolean;
  createdAt?: string;
}

export interface ProviderTestResult {
  success: boolean;
  error?: string;
  models?: string[];
}
