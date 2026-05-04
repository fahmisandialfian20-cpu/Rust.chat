# Frontend Guide

## 1. Frontend decision

Use SvelteKit + TypeScript.

The app has enough complexity that plain TS/CSS/JS will become difficult to maintain. SvelteKit provides routing, layouts, TypeScript integration, and a clear app structure.

## 2. Route structure

```text
src/routes/
  +layout.svelte
  +layout.ts
  +page.svelte

  login/+page.svelte
  register/[inviteToken]/+page.svelte

  lobby/+page.svelte

  spaces/[spaceId]/+layout.svelte
  spaces/[spaceId]/+page.svelte
  spaces/[spaceId]/channels/[channelId]/+page.svelte

  admin/+layout.svelte
  admin/spaces/+page.svelte
  admin/spaces/[spaceId]/channels/+page.svelte
  admin/spaces/[spaceId]/roles/+page.svelte
  admin/spaces/[spaceId]/members/+page.svelte
  admin/audit/+page.svelte

  settings/profile/+page.svelte
  settings/theme/+page.svelte
```

## 3. Component structure

```text
src/lib/components/
  layout/
    AppShell.svelte
    Sidebar.svelte
    TopBar.svelte

  chat/
    ChatShell.svelte
    MessageList.svelte
    MessageItem.svelte
    MessageComposer.svelte
    AttachmentPreview.svelte
    TypingIndicator.svelte

  spaces/
    SpaceSwitcher.svelte
    SpaceCard.svelte

  channels/
    ChannelList.svelte
    ChannelItem.svelte
    ChannelSettingsDialog.svelte
    ChannelFeatureTogglePanel.svelte

  permissions/
    PermissionChecklist.svelte
    RoleEditor.svelte
    ChannelOverrideEditor.svelte

  media/
    VoicePanel.svelte
    VideoRoom.svelte

  settings/
    ProfileSettings.svelte
    ThemeSettings.svelte
```

## 4. State structure

```text
src/lib/stores/
  session.svelte.ts
  realtime.svelte.ts
  presence.svelte.ts
  theme.svelte.ts
  active-space.svelte.ts
```

State rules:

- frontend can hide UI based on effective permissions;
- backend remains the source of truth;
- every action must handle `permission_denied`;
- WebSocket reconnect must reload active space/channel state.

## 5. API client

```text
src/lib/api/
  client.ts
  generated.d.ts
  errors.ts
```

Use generated OpenAPI types when available.

Client responsibilities:

- attach credentials/session;
- parse API errors;
- provide typed methods;
- handle `401` and `403`.

## 6. Realtime client

```text
src/lib/realtime/
  ws-client.ts
  event-router.ts
  reconnect.ts
```

Interface:

```ts
export class WsClient {
  connect(): Promise<void>;
  disconnect(): void;
  send<T>(type: string, payload: T, requestId?: string): void;
  on<T>(type: string, handler: (payload: T) => void): () => void;
}
```

Required behaviors:

- reconnect with backoff;
- heartbeat;
- request id correlation;
- ignore duplicate message events;
- resubscribe active channel after reconnect;
- show offline/reconnecting UI.

## 7. Chat UI behavior

Message composer must:

- disable itself if user cannot send;
- show why it is disabled;
- handle server rejection;
- support multiline input;
- support file attachment only if feature enabled;
- send optimistic message only with a clear pending state.

Message list must:

- virtualize later if needed;
- load older messages by cursor;
- render deleted messages safely;
- escape message content;
- avoid arbitrary HTML.

## 8. Permission UI behavior

The role editor must use grouped checklists:

- Instance;
- Space;
- Channels;
- Messages;
- Files;
- Voice/Video;
- Moderation;
- Personalization;
- Integrations.

The channel override editor must show tri-state controls:

- inherit;
- allow;
- deny.

## 9. Theme and skin system

Users may customize:

- avatar;
- display name;
- theme mode;
- accent color;
- font family from whitelist;
- density;
- bubble radius;
- background preset.

Do not allow arbitrary CSS in MVP.

Use CSS variables:

```css
:root {
  --accent: 220 90% 56%;
  --radius: 0.75rem;
  --chat-density: 1;
}
```

Safe theme type:

```ts
type UserTheme = {
  mode: "light" | "dark" | "system";
  accentColor: string;
  fontFamily: "system" | "inter" | "roboto" | "mono";
  radius: "none" | "sm" | "md" | "lg" | "xl";
  density: "compact" | "normal" | "comfortable";
};
```

## 10. Recommended UI foundation

Use Tailwind and shadcn-svelte style components for:

- buttons;
- dialogs;
- dropdowns;
- tabs;
- cards;
- scroll areas;
- sidebars;
- forms;
- checkboxes;
- switches;
- tooltips;
- command menu.

Because shadcn-svelte components are copied into the project, agents can modify them directly.

## 11. Accessibility requirements

- keyboard navigable sidebars;
- visible focus states;
- accessible dialogs;
- labels for icon-only buttons;
- color contrast;
- reduced motion preference;
- screen reader-friendly form errors.

## 12. Frontend MVP screens

1. Bootstrap owner page.
2. Login page.
3. Invite register page.
4. Lobby page.
5. Space shell.
6. Channel chat page.
7. Space/channel admin page.
8. Role permission editor.
9. Profile/theme settings.
