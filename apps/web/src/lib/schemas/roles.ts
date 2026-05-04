import { z } from 'zod';
import { PermissionKeySchema } from './permissions';

export const RoleSchema = z.object({
  id: z.string(),
  space_id: z.string(),
  name: z.string(),
  is_default: z.boolean(),
  permissions: z.array(PermissionKeySchema),
});

export type Role = z.infer<typeof RoleSchema>;

export const CreateRoleSchema = z.object({
  name: z.string().min(1, 'Role name is required').max(64, 'Role name is too long'),
  permission_keys: z.array(PermissionKeySchema),
});

export type CreateRoleInput = z.infer<typeof CreateRoleSchema>;

export const UpdateRoleSchema = CreateRoleSchema.partial();

export type UpdateRoleInput = z.infer<typeof UpdateRoleSchema>;
