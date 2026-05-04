import { browser } from '$app/environment';
import { DEFAULT_THEME, ThemeSchema, type ThemePreferences } from '$lib/schemas/theme';

const STORAGE_KEY = 'rustchat-theme';

function loadFromStorage(): ThemePreferences {
  if (!browser) return { ...DEFAULT_THEME };
  try {
    const raw = localStorage.getItem(STORAGE_KEY);
    if (!raw) return { ...DEFAULT_THEME };
    const parsed = JSON.parse(raw);
    const result = ThemeSchema.safeParse(parsed);
    if (result.success) return result.data;
  } catch {
    // corrupted data, fall back to default
  }
  return { ...DEFAULT_THEME };
}

function persistToStorage(prefs: ThemePreferences): void {
  if (!browser) return;
  try {
    localStorage.setItem(STORAGE_KEY, JSON.stringify(prefs));
  } catch {
    // storage unavailable
  }
}

function applyTheme(prefs: ThemePreferences): void {
  if (!browser) return;
  const root = document.documentElement;
  root.setAttribute('data-theme', prefs.mode);
  root.setAttribute('data-accent', prefs.accent);
  root.setAttribute('data-density', prefs.density);
  root.setAttribute('data-message-display', prefs.message_display);
}

let current = $state<ThemePreferences>(loadFromStorage());

applyTheme(current);

export function getThemePreferences(): ThemePreferences {
  return current;
}

export function setMode(mode: ThemePreferences['mode']): void {
  current = { ...current, mode };
  persistToStorage(current);
  applyTheme(current);
}

export function setAccent(accent: ThemePreferences['accent']): void {
  current = { ...current, accent };
  persistToStorage(current);
  applyTheme(current);
}

export function setDensity(density: ThemePreferences['density']): void {
  current = { ...current, density };
  persistToStorage(current);
  applyTheme(current);
}

export function setMessageDisplay(message_display: ThemePreferences['message_display']): void {
  current = { ...current, message_display };
  persistToStorage(current);
  applyTheme(current);
}

export function resetToDefaults(): void {
  current = { ...DEFAULT_THEME };
  persistToStorage(current);
  applyTheme(current);
}
