let accessToken = $state<string | null>(null);
let refreshToken = $state<string | null>(null);
let user = $state<unknown>(null);

export function getAccessToken(): string | null {
  return accessToken;
}

export function getRefreshToken(): string | null {
  return refreshToken;
}

export function getUser(): unknown {
  return user;
}

export function setAuth(response: { user: unknown; access_token: string; refresh_token: string }): void {
  accessToken = response.access_token;
  refreshToken = response.refresh_token;
  user = response.user;
}

export function clearAuth(): void {
  accessToken = null;
  refreshToken = null;
  user = null;
}
