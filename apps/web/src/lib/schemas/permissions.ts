import { z } from 'zod';

export const PERMISSION_KEYS = [
  'manage_instance',
  'manage_spaces',
  'manage_roles',
  'manage_members',
  'manage_channels',
  'manage_invites',
  'view_audit_log',

  'view_space',
  'view_channel',
  'read_messages',

  'send_messages',
  'edit_own_message',
  'delete_own_message',
  'edit_any_message',
  'delete_any_message',
  'pin_messages',
  'mention_everyone',

  'send_files',
  'create_threads',
  'manage_threads',
  'add_reactions',

  'join_voice',
  'start_voice',
  'join_video',
  'start_video',
  'share_screen',

  'kick_members',
  'ban_members',
  'mute_members',
  'manage_moderation',

  'customize_own_profile',
  'customize_space',
  'use_webhooks',
] as const;

export type PermissionKey = (typeof PERMISSION_KEYS)[number];

export const PermissionKeySchema = z.enum(PERMISSION_KEYS);

export interface PermissionGroup {
  label: string;
  keys: PermissionKey[];
}

export const PERMISSION_GROUPS: PermissionGroup[] = [
  {
    label: 'Instance / Admin',
    keys: [
      'manage_instance',
      'manage_spaces',
      'manage_roles',
      'manage_members',
      'manage_channels',
      'manage_invites',
      'view_audit_log',
    ],
  },
  {
    label: 'Space / Channel',
    keys: ['view_space', 'view_channel', 'read_messages'],
  },
  {
    label: 'Messages',
    keys: [
      'send_messages',
      'edit_own_message',
      'delete_own_message',
      'edit_any_message',
      'delete_any_message',
      'pin_messages',
      'mention_everyone',
    ],
  },
  {
    label: 'Files / Threads / Reactions',
    keys: ['send_files', 'create_threads', 'manage_threads', 'add_reactions'],
  },
  {
    label: 'Media',
    keys: ['join_voice', 'start_voice', 'join_video', 'start_video', 'share_screen'],
  },
  {
    label: 'Moderation',
    keys: ['kick_members', 'ban_members', 'mute_members', 'manage_moderation'],
  },
  {
    label: 'Profile / Customization',
    keys: ['customize_own_profile', 'customize_space', 'use_webhooks'],
  },
];
