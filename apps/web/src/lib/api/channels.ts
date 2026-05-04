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
  voice_group_enabled: boolean;
  video_group_enabled: boolean;
  threads_enabled: boolean;
  reactions_enabled: boolean;
}

export interface UpdateChannelData {
  name?: string;
  topic?: string | null;
  visibility?: 'Public' | 'Private';
}

export async function listChannels(spaceId: string): Promise<Channel[]> {
  const token = getAccessToken();
  if (!token) {
    throw { status: 401, message: 'Not authenticated' } satisfies ApiError;
  }

  const response = await fetch(apiUrl(`/api/v1/spaces/${spaceId}/channels`), {
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

export async function updateChannel(spaceId: string, channelId: string, data: UpdateChannelData): Promise<Channel> {
  const token = getAccessToken();
  if (!token) {
    throw { status: 401, message: 'Not authenticated' } satisfies ApiError;
  }

  const response = await fetch(apiUrl(`/api/v1/spaces/${spaceId}/channels/${channelId}`), {
    method: 'PUT',
    headers: {
      'Content-Type': 'application/json',
      Authorization: `Bearer ${token}`,
    },
    body: JSON.stringify(data),
  });

  if (!response.ok) {
    const body = await response.json().catch(() => ({}));
    throw { status: response.status, message: body.message ?? 'Request failed' } satisfies ApiError;
  }

  const responseData: unknown = await response.json();
  const result = ChannelSchema.safeParse(responseData);
  if (!result.success) {
    throw { status: 500, message: 'Invalid response from server' } satisfies ApiError;
  }

  return result.data;
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

export async function updateChannelFlags(spaceId: string, channelId: string, flags: FeatureFlags): Promise<FeatureFlags> {
  const token = getAccessToken();
  if (!token) {
    throw { status: 401, message: 'Not authenticated' } satisfies ApiError;
  }

  const response = await fetch(apiUrl(`/api/v1/spaces/${spaceId}/channels/${channelId}/feature-flags`), {
    method: 'PUT',
    headers: {
      'Content-Type': 'application/json',
      Authorization: `Bearer ${token}`,
    },
    body: JSON.stringify(flags),
  });

  if (!response.ok) {
    const body = await response.json().catch(() => ({}));
    throw { status: response.status, message: body.message ?? 'Request failed' } satisfies ApiError;
  }

  const data: unknown = await response.json();
  return data as FeatureFlags;
}
