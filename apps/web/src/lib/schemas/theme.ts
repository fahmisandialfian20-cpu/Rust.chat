import { z } from 'zod';

export const MODE_OPTIONS = ['dark', 'light', 'system'] as const;
export const ACCENT_OPTIONS = ['purple', 'blue', 'green', 'orange', 'pink'] as const;
export const DENSITY_OPTIONS = ['comfortable', 'compact'] as const;
export const MESSAGE_DISPLAY_OPTIONS = ['cozy', 'compact'] as const;

export const ThemeSchema = z.object({
  mode: z.enum(MODE_OPTIONS),
  accent: z.enum(ACCENT_OPTIONS),
  density: z.enum(DENSITY_OPTIONS),
  message_display: z.enum(MESSAGE_DISPLAY_OPTIONS),
});

export type ThemePreferences = z.infer<typeof ThemeSchema>;

export const DEFAULT_THEME: ThemePreferences = {
  mode: 'dark',
  accent: 'purple',
  density: 'comfortable',
  message_display: 'cozy',
};

export const ACCENT_LABELS: Record<string, string> = {
  purple: 'Purple',
  blue: 'Blue',
  green: 'Green',
  orange: 'Orange',
  pink: 'Pink',
};
