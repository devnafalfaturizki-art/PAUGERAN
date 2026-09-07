import type { z } from 'zod';

export const caseValidator = z.object({
  title: z.string().min(1).max(500),
  state: z.string().optional(),
  mode: z.string().optional(),
});

export type CaseInput = z.infer<typeof caseValidator>;
