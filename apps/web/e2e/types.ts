export interface AuthResponse {
  user: { id: string; username: string };
  access_token: string;
  refresh_token: string;
}

export interface Space {
  id: string;
  name: string;
  slug: string;
  description: string | null;
}

export interface Channel {
  id: string;
  space_id: string;
  name: string;
  slug: string;
  kind: string;
  visibility: string;
}

export interface RoleWithPermissions {
  role: {
    id: string;
    space_id: string;
    name: string;
    is_default: boolean;
  };
  permission_keys: string[];
}

export interface Message {
  id: string;
  channel_id: string;
  author_user_id: string;
  content: string;
}

export interface Invite {
  id: string;
  code: string;
  space_id: string | null;
  max_uses: number | null;
  used_count: number;
}
