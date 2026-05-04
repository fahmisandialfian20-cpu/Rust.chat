import { apiUrl } from '$lib/config';
import { getAccessToken } from '$lib/stores/auth.svelte';
import { ChannelSchema } from '$lib/schemas/channels';
import type { Channel } from '$lib/schemas/channels';

export interface ApiError {
  status: number;
  message: string;
}

export async function getChannel(spaceId: string, channelId: string): Promise<Channel> {
  const token = getAccessToken();
  if (!token) {
    throw { status: 401, message: 'Not authenticated' } satisfies ApiError;
  }

  const response = await fetch(apiUrl(`/api/v1/spaces/${spaceId}/channels/${channelId}`), {
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
  const result = ChannelSchema.safeParse(data);
  if (!result.success) {
    throw { status: 500, message: 'Invalid response from server' } satisfies ApiError;
  }

  return result.data;
}

export async function listVisibleChannels(spaceId: string): Promise<Channel[]> {
  const token = getAccessToken();
  if (!token) {
    throw { status: 401, message: 'Not authenticated' } satisfies ApiError;
  }

  const response = await fetch(apiUrl(`/api/v1/spaces/${spaceId}/channels/visible`), {
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
  const result = ChannelSchema.array().safeParse(data);
  if (!result.success) {
    throw { status: 500, message: 'Invalid response from server' } satisfies ApiError;
  }

  return result.data;
}

export interface FeatureFlags {
  text_enabled: boolean;
  file_upload_enabled: boolean;
  voice_enabled: boolean;
  video_enabled: boolean;
}

export async function getChannelFlags(spaceId: string, channelId: string): Promise<FeatureFlags> {
  const token = getAccessToken();
  if (!token) {
    throw { status: 401, message: 'Not authenticated' } satisfies ApiError;
  }

  const response = await fetch(apiUrl(`/api/v1/spaces/${spaceId}/channels/${channelId}/feature-flags`), {
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
  return data as FeatureFlags;
}
