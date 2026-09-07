import type { z } from 'zod';

export const messageValidator = z.object({
  role: z.enum(['system', 'user']),
  content: z.string().min(1),
});

export type MessageInput = z.infer<typeof messageValidator>;
