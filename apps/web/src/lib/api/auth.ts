import { apiUrl } from '$lib/config';
import type { BootstrapInput, LoginInput, RegisterInput } from '$lib/schemas/auth';

export interface AuthResponse {
  user: unknown;
  access_token: string;
  refresh_token: string;
}

export interface ApiError {
  status: number;
  message: string;
}

async function authFetch(url: string, body: unknown): Promise<AuthResponse> {
  const response = await fetch(url, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(body),
  });

  if (!response.ok) {
    const body = await response.json().catch(() => ({}));
    throw { status: response.status, message: body.message ?? 'Request failed' } satisfies ApiError;
  }

  const data: AuthResponse = await response.json();
  return data;
}

export async function bootstrapOwner(input: BootstrapInput): Promise<AuthResponse> {
  return authFetch(apiUrl('/api/v1/auth/bootstrap-owner'), input);
}

export async function login(input: LoginInput): Promise<AuthResponse> {
  return authFetch(apiUrl('/api/v1/auth/login'), input);
}

export async function register(input: RegisterInput): Promise<AuthResponse> {
  return authFetch(apiUrl('/api/v1/auth/register'), input);
}
