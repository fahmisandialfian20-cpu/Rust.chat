import { apiUrl } from '$lib/config';
import { getAccessToken } from '$lib/stores/auth.svelte';
import { MessageSchema } from '$lib/schemas/messages';
import type { Message } from '$lib/schemas/messages';
import type { ApiError } from './channels';

export interface ListMessagesParams {
  limit?: number;
  before?: string;
}

export async function listMessages(channelId: string, params?: ListMessagesParams): Promise<Message[]> {
  const token = getAccessToken();
  if (!token) {
    throw { status: 401, message: 'Not authenticated' } satisfies ApiError;
  }

  const url = new URL(apiUrl(`/api/v1/channels/${channelId}/messages`));
  if (params?.limit != null) url.searchParams.set('limit', String(params.limit));
  if (params?.before) url.searchParams.set('before', params.before);

  const response = await fetch(url.toString(), {
    method: 'GET',
    headers: {
      'Content-Type': 'application/json',
      Authorization: `Bearer ${token}`,
    },
  });

  if (!response.ok) {
    const body = await response.json().catch(() => ({}));
    throw { status: response.status, message: body.message ?? 'Request failed' } satisfies ApiError;
  }

  const data: unknown = await response.json();
  const result = MessageSchema.array().safeParse(data);
  if (!result.success) {
    throw { status: 500, message: 'Invalid response from server' } satisfies ApiError;
  }

  return result.data;
}

export async function sendMessage(channelId: string, content: string): Promise<Message> {
  const token = getAccessToken();
  if (!token) {
    throw { status: 401, message: 'Not authenticated' } satisfies ApiError;
  }

  const response = await fetch(apiUrl(`/api/v1/channels/${channelId}/messages`), {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
      Authorization: `Bearer ${token}`,
    },
    body: JSON.stringify({ content }),
  });

  if (!response.ok) {
    const body = await response.json().catch(() => ({}));
    throw { status: response.status, message: body.message ?? 'Request failed' } satisfies ApiError;
  }

  const data: unknown = await response.json();
  const result = MessageSchema.safeParse(data);
  if (!result.success) {
    throw { status: 500, message: 'Invalid response from server' } satisfies ApiError;
  }

  return result.data;
}
