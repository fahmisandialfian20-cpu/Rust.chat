import { z } from 'zod';

export const ChannelKindSchema = z.enum(['Text', 'Voice', 'Video']);
export type ChannelKind = z.infer<typeof ChannelKindSchema>;

export const ChannelVisibilitySchema = z.enum(['Public', 'Private']);
export type ChannelVisibility = z.infer<typeof ChannelVisibilitySchema>;

export const ChannelSchema = z.object({
  id: z.string(),
  space_id: z.string(),
  name: z.string(),
  slug: z.string(),
  kind: ChannelKindSchema,
  visibility: ChannelVisibilitySchema,
  position: z.number(),
  topic: z.string().nullable(),
  created_at: z.string(),
  updated_at: z.string(),
});

export type Channel = z.infer<typeof ChannelSchema>;
