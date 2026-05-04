import type { Role, CreateRoleInput, UpdateRoleInput } from '$lib/schemas/roles';
import type { PermissionKey } from '$lib/schemas/permissions';

export interface ApiError {
  status: number;
  message: string;
}

const BACKEND_UNAVAILABLE_MSG =
  'Not available: backend role CRUD not implemented';

function notAvailableError(): never {
  throw { status: 501, message: BACKEND_UNAVAILABLE_MSG } satisfies ApiError;
}

export async function listRoles(_spaceId: string): Promise<Role[]> {
  notAvailableError();
}

export async function getRole(_spaceId: string, _roleId: string): Promise<Role> {
  notAvailableError();
}

export async function createRole(
  _spaceId: string,
  _input: CreateRoleInput,
): Promise<Role> {
  notAvailableError();
}

export async function updateRole(
  _spaceId: string,
  _roleId: string,
  _input: UpdateRoleInput,
): Promise<Role> {
  notAvailableError();
}

export async function deleteRole(
  _spaceId: string,
  _roleId: string,
): Promise<void> {
  notAvailableError();
}

export async function setRolePermissions(
  _spaceId: string,
  _roleId: string,
  _permissionKeys: PermissionKey[],
): Promise<Role> {
  notAvailableError();
}
