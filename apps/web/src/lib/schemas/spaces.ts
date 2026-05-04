import { z } from 'zod';

export const SpaceVisibilitySchema = z.enum(['Public', 'Private']);
export type SpaceVisibility = z.infer<typeof SpaceVisibilitySchema>;

export const SpaceSchema = z.object({
  id: z.string(),
  name: z.string(),
  slug: z.string(),
  description: z.string().nullable(),
  icon_object_id: z.string().nullable(),
  created_by: z.string(),
  visibility: SpaceVisibilitySchema,
  settings: z.record(z.string(), z.unknown()),
  created_at: z.string(),
  updated_at: z.string(),
});

export type Space = z.infer<typeof SpaceSchema>;
