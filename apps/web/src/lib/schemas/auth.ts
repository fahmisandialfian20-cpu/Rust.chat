import { z } from 'zod';

export const BootstrapSchema = z.object({
  username: z
    .string()
    .trim()
    .min(3, 'Username must be at least 3 characters')
    .regex(/^\S+$/, 'Username must not contain spaces'),
  password: z.string().min(6, 'Password must be at least 6 characters'),
});

export type BootstrapInput = z.infer<typeof BootstrapSchema>;

const ClientMetadataSchema = z.object({
  client_type: z.string(),
  platform: z.string(),
});

export const LoginSchema = z.object({
  username_or_email: z.string().trim().min(1, 'Username or email is required'),
  password: z.string().min(1, 'Password is required'),
  client_metadata: ClientMetadataSchema.optional(),
});

export type LoginInput = z.infer<typeof LoginSchema>;

export const RegisterSchema = z.object({
  username: z
    .string()
    .trim()
    .min(3, 'Username must be at least 3 characters')
    .regex(/^\S+$/, 'Username must not contain spaces'),
  password: z.string().min(6, 'Password must be at least 6 characters'),
  invite_code: z.string().optional(),
});

export type RegisterInput = z.infer<typeof RegisterSchema>;
