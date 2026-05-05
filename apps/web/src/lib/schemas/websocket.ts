import { z } from 'zod';
import { MessageSchema } from './messages';
import { ChannelSchema } from './channels';

export const websocketEnvelopeSchema = z.object({
  version: z.literal(1),
  type: z.string().min(1),
  request_id: z.string().min(1).optional(),
  payload: z.unknown().default({}),
  sent_at: z.string().datetime().optional()
});

export type WebsocketEnvelope = z.infer<typeof websocketEnvelopeSchema>;

export const WS_EVENT_TYPES = [
  'hello.ok',
  'message.created',
  'message.updated',
  'message.deleted',
  'typing.updated',
  'presence.updated',
  'channel.created',
  'channel.updated',
  'channel.deleted',
  'permission.updated',
  'member.joined',
  'member.left',
  'notification.created',
  'media.room.updated',
  'error',
] as const;

export type WsEventType = (typeof WS_EVENT_TYPES)[number];

export const wsEventTypeSchema = z.enum(WS_EVENT_TYPES);

export const helloOkPayloadSchema = z.object({
  user_id: z.string().min(1),
  session_id: z.string().min(1),
});
export type HelloOkPayload = z.infer<typeof helloOkPayloadSchema>;

export const messageUpdatedPayloadSchema = z.object({
  message_id: z.string().min(1),
  content: z.string(),
});
export type MessageUpdatedPayload = z.infer<typeof messageUpdatedPayloadSchema>;

export const messageDeletedPayloadSchema = z.object({
  message_id: z.string().min(1),
});
export type MessageDeletedPayload = z.infer<typeof messageDeletedPayloadSchema>;

export const typingUpdatedPayloadSchema = z.object({
  channel_id: z.string().min(1),
  user_id: z.string().min(1),
  is_typing: z.boolean(),
});
export type TypingUpdatedPayload = z.infer<typeof typingUpdatedPayloadSchema>;

export const presenceUpdatedPayloadSchema = z.object({
  user_id: z.string().min(1),
  status: z.enum(['online', 'offline', 'idle']),
});
export type PresenceUpdatedPayload = z.infer<typeof presenceUpdatedPayloadSchema>;

export const permissionUpdatedPayloadSchema = z.object({
  user_id: z.string().min(1),
  permission: z.string().min(1),
  allowed: z.boolean(),
});
export type PermissionUpdatedPayload = z.infer<typeof permissionUpdatedPayloadSchema>;

export const memberJoinedPayloadSchema = z.object({
  space_id: z.string().min(1),
  user_id: z.string().min(1),
});
export type MemberJoinedPayload = z.infer<typeof memberJoinedPayloadSchema>;

export const memberLeftPayloadSchema = z.object({
  space_id: z.string().min(1),
  user_id: z.string().min(1),
});
export type MemberLeftPayload = z.infer<typeof memberLeftPayloadSchema>;

export const notificationCreatedPayloadSchema = z.object({
  type: z.string().min(1),
  title: z.string().min(1),
  body: z.string(),
});
export type NotificationCreatedPayload = z.infer<typeof notificationCreatedPayloadSchema>;

export const mediaRoomUpdatedPayloadSchema = z.object({
  room_id: z.string().min(1),
  status: z.string().min(1),
});
export type MediaRoomUpdatedPayload = z.infer<typeof mediaRoomUpdatedPayloadSchema>;

export const errorPayloadSchema = z.object({
  code: z.string().min(1),
  message: z.string().min(1),
  details: z.unknown().optional(),
});
export type ErrorPayload = z.infer<typeof errorPayloadSchema>;

export const channelDeletedPayloadSchema = z.object({
  channel_id: z.string().min(1),
});
export type ChannelDeletedPayload = z.infer<typeof channelDeletedPayloadSchema>;

type EventPayloadMap = {
  'hello.ok': HelloOkPayload;
  'message.created': z.infer<typeof MessageSchema>;
  'message.updated': MessageUpdatedPayload;
  'message.deleted': MessageDeletedPayload;
  'typing.updated': TypingUpdatedPayload;
  'presence.updated': PresenceUpdatedPayload;
  'channel.created': z.infer<typeof ChannelSchema>;
  'channel.updated': z.infer<typeof ChannelSchema>;
  'channel.deleted': ChannelDeletedPayload;
  'permission.updated': PermissionUpdatedPayload;
  'member.joined': MemberJoinedPayload;
  'member.left': MemberLeftPayload;
  'notification.created': NotificationCreatedPayload;
  'media.room.updated': MediaRoomUpdatedPayload;
  'error': ErrorPayload;
};

export type TypedWsEvent = {
  [K in keyof EventPayloadMap]: {
    version: 1;
    type: K;
    payload: EventPayloadMap[K];
    request_id?: string;
    sent_at?: string;
  }
}[keyof EventPayloadMap];

const legacyEventSchema = z.object({
  type: wsEventTypeSchema,
  data: z.unknown().default({}),
});

function normalizeLegacy(raw: unknown): unknown {
  const legacy = legacyEventSchema.safeParse(raw);
  if (!legacy.success) return null;
  return {
    version: 1,
    type: legacy.data.type,
    payload: legacy.data.data,
  };
}

const payloadSchemaByType: Record<string, z.ZodType<unknown>> = {
  'hello.ok': helloOkPayloadSchema,
  'message.created': MessageSchema,
  'message.updated': messageUpdatedPayloadSchema,
  'message.deleted': messageDeletedPayloadSchema,
  'typing.updated': typingUpdatedPayloadSchema,
  'presence.updated': presenceUpdatedPayloadSchema,
  'channel.created': ChannelSchema,
  'channel.updated': ChannelSchema,
  'channel.deleted': channelDeletedPayloadSchema,
  'permission.updated': permissionUpdatedPayloadSchema,
  'member.joined': memberJoinedPayloadSchema,
  'member.left': memberLeftPayloadSchema,
  'notification.created': notificationCreatedPayloadSchema,
  'media.room.updated': mediaRoomUpdatedPayloadSchema,
  'error': errorPayloadSchema,
};

export interface ParseWsEventSuccess {
  success: true;
  data: TypedWsEvent;
}

export interface ParseWsEventError {
  success: false;
  error: string;
}

export type ParseWsEventResult = ParseWsEventSuccess | ParseWsEventError;

export function parseWsEvent(raw: unknown): ParseWsEventResult {
  const envelope = websocketEnvelopeSchema.safeParse(raw);
  if (!envelope.success) {
    const normalized = normalizeLegacy(raw);
    if (normalized) {
      return parseWsEvent(normalized);
    }
    return { success: false, error: 'Invalid WebSocket event envelope' };
  }

  const { type, payload, request_id, sent_at } = envelope.data;

  const typeResult = wsEventTypeSchema.safeParse(type);
  if (!typeResult.success) {
    return { success: false, error: `Unknown event type: "${type}"` };
  }

  const payloadSchema = payloadSchemaByType[type];
  const payloadResult = payloadSchema.safeParse(payload);
  if (!payloadResult.success) {
    return {
      success: false,
      error: `Invalid payload for event "${type}": ${payloadResult.error.message}`,
    };
  }

  return {
    success: true,
    data: {
      version: 1,
      type: type as TypedWsEvent['type'],
      payload: payloadResult.data,
      ...(request_id !== undefined && { request_id }),
      ...(sent_at !== undefined && { sent_at }),
    } as TypedWsEvent,
  };
}
