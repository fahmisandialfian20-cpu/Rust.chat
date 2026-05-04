import { z } from 'zod';

export const MessageKindSchema = z.enum(['Default', 'System']);
export type MessageKind = z.infer<typeof MessageKindSchema>;

export const MessageSchema = z.object({
  id: z.string(),
  channel_id: z.string(),
  author_user_id: z.string(),
  content: z.string(),
  kind: MessageKindSchema,
  reply_to_message_id: z.string().nullable().optional(),
  edited_at: z.string().nullable().optional(),
  deleted_at: z.string().nullable().optional(),
  created_at: z.string(),
});

export type Message = z.infer<typeof MessageSchema>;

export const CreateMessageSchema = z.object({
  content: z.string().min(1).trim(),
  kind: MessageKindSchema.optional(),
  reply_to_message_id: z.string().optional(),
});

export type CreateMessagePayload = z.infer<typeof CreateMessageSchema>;
