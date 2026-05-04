import { apiUrl } from '$lib/config';
import { getAccessToken } from '$lib/stores/auth.svelte';
import type { Space } from '$lib/schemas/spaces';
import { SpaceSchema } from '$lib/schemas/spaces';

export interface ApiError {
  status: number;
  message: string;
}

interface ListSpacesParams {
  limit?: number;
  offset?: number;
}

export async function getSpace(spaceId: string): Promise<Space> {
  const token = getAccessToken();
  if (!token) {
    throw { status: 401, message: 'Not authenticated' } satisfies ApiError;
  }

  const response = await fetch(apiUrl(`/api/v1/spaces/${spaceId}`), {
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
  const result = SpaceSchema.safeParse(data);
  if (!result.success) {
    throw { status: 500, message: 'Invalid response from server' } satisfies ApiError;
  }

  return result.data;
}

export async function listSpaces(params?: ListSpacesParams): Promise<Space[]> {
  const token = getAccessToken();
  if (!token) {
    throw { status: 401, message: 'Not authenticated' } satisfies ApiError;
  }

  const url = new URL(apiUrl('/api/v1/spaces'));
  if (params?.limit != null) url.searchParams.set('limit', String(params.limit));
  if (params?.offset != null) url.searchParams.set('offset', String(params.offset));

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
  const result = SpaceSchema.array().safeParse(data);
  if (!result.success) {
    throw { status: 500, message: 'Invalid response from server' } satisfies ApiError;
  }

  return result.data;
}
