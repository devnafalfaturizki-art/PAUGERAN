export interface Message {
  id?: string;
  role: 'system' | 'user';
  content: string;
  certaintyScore?: number;
  createdAt?: string;
}
