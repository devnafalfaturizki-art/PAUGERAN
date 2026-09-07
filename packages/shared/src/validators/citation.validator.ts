import type { z } from 'zod';

export const citationValidator = z.object({
  sourceType: z.string(),
  number: z.string().optional(),
  year: z.string().optional(),
  title: z.string().min(1),
  article: z.string().optional(),
  enactedOn: z.string().optional(),
  status: z.enum(['active', 'revoked', 'amended']),
  fullText: z.string().min(1),
});

export type CitationInput = z.infer<typeof citationValidator>;
