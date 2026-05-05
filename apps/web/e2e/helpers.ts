import { type APIRequestContext, expect } from '@playwright/test';
import type { AuthResponse, Space, Channel, RoleWithPermissions, Message, Invite } from './types';

const API_BASE = 'http://localhost:3000/api/v1';

export async function bootstrapOwner(request: APIRequestContext): Promise<AuthResponse> {
  const res = await request.post(`${API_BASE}/auth/bootstrap-owner`, {
    data: { username: 'hoster', password: 'password123' },
  });
  expect(res.ok()).toBeTruthy();
  return res.json();
}

export async function registerUser(
  request: APIRequestContext,
  username: string,
  password: string,
  inviteCode?: string,
): Promise<AuthResponse> {
  const res = await request.post(`${API_BASE}/auth/register`, {
    data: { username, password, invite_code: inviteCode },
  });
  expect(res.ok()).toBeTruthy();
  return res.json();
}

export async function createSpace(
  request: APIRequestContext,
  token: string,
  name: string,
  description?: string,
): Promise<Space> {
  const res = await request.post(`${API_BASE}/spaces`, {
    headers: { Authorization: `Bearer ${token}` },
    data: { name, description: description ?? null },
  });
  expect(res.ok()).toBeTruthy();
  return res.json();
}

export async function createChannel(
  request: APIRequestContext,
  token: string,
  spaceId: string,
  name: string,
  visibility: 'public' | 'private',
): Promise<Channel> {
  const res = await request.post(`${API_BASE}/spaces/${spaceId}/channels`, {
    headers: { Authorization: `Bearer ${token}` },
    data: { name, kind: 'Text', visibility },
  });
  expect(res.ok()).toBeTruthy();
  return res.json();
}

export async function sendMessage(
  request: APIRequestContext,
  token: string,
  channelId: string,
  content: string,
): Promise<Message> {
  const res = await request.post(`${API_BASE}/channels/${channelId}/messages`, {
    headers: { Authorization: `Bearer ${token}` },
    data: { content },
  });
  return res.json();
}

export async function listMessages(
  request: APIRequestContext,
  token: string,
  channelId: string,
): Promise<Message[]> {
  const res = await request.get(`${API_BASE}/channels/${channelId}/messages`, {
    headers: { Authorization: `Bearer ${token}` },
  });
  expect(res.ok()).toBeTruthy();
  return res.json();
}

export async function createInvite(
  request: APIRequestContext,
  token: string,
  spaceId: string,
): Promise<Invite> {
  const res = await request.post(`${API_BASE}/invites`, {
    headers: { Authorization: `Bearer ${token}` },
    data: { space_id: spaceId, max_uses: 10 },
  });
  expect(res.ok()).toBeTruthy();
  return res.json();
}

export async function acceptInvite(
  request: APIRequestContext,
  token: string,
  code: string,
): Promise<void> {
  const res = await request.post(`${API_BASE}/invites/${code}/accept`, {
    headers: { Authorization: `Bearer ${token}` },
  });
  expect(res.ok()).toBeTruthy();
}

export async function createRole(
  request: APIRequestContext,
  token: string,
  spaceId: string,
  name: string,
  permissionKeys: string[],
): Promise<RoleWithPermissions> {
  const res = await request.post(`${API_BASE}/spaces/${spaceId}/roles`, {
    headers: { Authorization: `Bearer ${token}` },
    data: { name, permission_keys: permissionKeys },
  });
  expect(res.ok()).toBeTruthy();
  return res.json();
}

export async function assignRole(
  request: APIRequestContext,
  token: string,
  spaceId: string,
  userId: string,
  roleId: string,
): Promise<void> {
  const res = await request.post(`${API_BASE}/spaces/${spaceId}/members/${userId}/roles`, {
    headers: { Authorization: `Bearer ${token}` },
    data: { role_id: roleId },
  });
  expect(res.ok()).toBeTruthy();
}

export async function updateRole(
  request: APIRequestContext,
  token: string,
  spaceId: string,
  roleId: string,
  permissionKeys: string[],
): Promise<RoleWithPermissions> {
  const res = await request.put(`${API_BASE}/spaces/${spaceId}/roles/${roleId}`, {
    headers: { Authorization: `Bearer ${token}` },
    data: { permission_keys: permissionKeys },
  });
  expect(res.ok()).toBeTruthy();
  return res.json();
}

export async function listVisibleChannels(
  request: APIRequestContext,
  token: string,
  spaceId: string,
): Promise<Channel[]> {
  const res = await request.get(`${API_BASE}/spaces/${spaceId}/channels/visible`, {
    headers: { Authorization: `Bearer ${token}` },
  });
  expect(res.ok()).toBeTruthy();
  return res.json();
}
