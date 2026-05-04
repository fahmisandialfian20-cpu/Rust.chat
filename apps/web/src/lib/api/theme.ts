import type { ThemePreferences } from '$lib/schemas/theme';

export interface ApiError {
  status: number;
  message: string;
}

const BACKEND_NOT_AVAILABLE = 'Theme preference endpoints are not available on the server yet.';

export async function getTheme(): Promise<ThemePreferences> {
  throw { status: 501, message: BACKEND_NOT_AVAILABLE } satisfies ApiError;
}

export async function updateTheme(_prefs: ThemePreferences): Promise<ThemePreferences> {
  throw { status: 501, message: BACKEND_NOT_AVAILABLE } satisfies ApiError;
}
