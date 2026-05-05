# Frontend Channel Visibility Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Integrate backend permission system into frontend channel list so users only see channels they are authorized to view, with real-time WebSocket updates.

**Architecture:** Add `list_user_permissions()` to backend PermissionService + expose via REST handler. Add `channel.created/updated/deleted` WS events. Frontend subscribes to WS in space layout, shows/hides UI based on permissions.

**Tech Stack:** Rust (Axum + SQLx), SvelteKit 5 (runes), TypeScript (zod), Tailwind CSS

---

## Task 1: Backend — Add `list_user_permissions` to PermissionService

**Files:**
- Modify: `apps/server/src/permissions/service.rs`
- Modify: `apps/server/src/permissions/mod.rs`

- [ ] **Step 1: Add `list_user_permissions` method to PermissionService**

In `apps/server/src/permissions/service.rs`, add after the existing `has_any_permission` method:

```rust
pub async fn list_user_permissions(
    &self,
    user_id: Uuid,
    space_id: Uuid,
) -> Result<Vec<String>, AppError> {
    let all_keys = [
        PermissionKey::ManageInstance,
        PermissionKey::ManageSpaces,
        PermissionKey::ManageRoles,
        PermissionKey::ManageMembers,
        PermissionKey::ManageChannels,
        PermissionKey::ManageInvites,
        PermissionKey::ViewAuditLog,
        PermissionKey::ViewSpace,
        PermissionKey::ViewChannel,
        PermissionKey::ReadMessages,
        PermissionKey::SendMessages,
        PermissionKey::EditOwnMessage,
        PermissionKey::DeleteOwnMessage,
        PermissionKey::EditAnyMessage,
        PermissionKey::DeleteAnyMessage,
        PermissionKey::PinMessages,
        PermissionKey::MentionEveryone,
        PermissionKey::SendFiles,
        PermissionKey::CreateThreads,
        PermissionKey::ManageThreads,
        PermissionKey::AddReactions,
        PermissionKey::JoinVoice,
        PermissionKey::StartVoice,
        PermissionKey::JoinVideo,
        PermissionKey::StartVideo,
        PermissionKey::ShareScreen,
        PermissionKey::KickMembers,
        PermissionKey::BanMembers,
        PermissionKey::MuteMembers,
        PermissionKey::ManageModeration,
        PermissionKey::CustomizeOwnProfile,
        PermissionKey::CustomizeSpace,
        PermissionKey::UseWebhooks,
    ];

    let mut allowed = Vec::new();
    for key in all_keys {
        if self
            .resolver
            .check(user_id, key, Some(space_id), None)
            .await?
            .is_allowed()
        {
            allowed.push(key.as_str().to_string());
        }
    }
    Ok(allowed)
}
```

Add import for `PermissionKey` at the top if not already imported:
```rust
use super::keys::PermissionKey;
```
(It may already be imported from the resolver — verify exists)

- [ ] **Step 2: Verify it compiles**

Run: `cd apps/server && cargo check`
Expected: Compiles without errors

---

## Task 2: Backend — Create `get_my_permissions` handler + route

**Files:**
- Create: `apps/server/src/handlers/permissions.rs`
- Modify: `apps/server/src/handlers/mod.rs`
- Modify: `apps/server/src/main.rs`

- [ ] **Step 1: Create `apps/server/src/handlers/permissions.rs`**

```rust
use axum::{
    extract::{Path, State},
    response::Json,
};
use uuid::Uuid;

use crate::auth::middleware::AuthUser;
use crate::error::AppError;
use crate::state::AppState;

pub async fn get_my_permissions(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(space_id): Path<Uuid>,
) -> Result<Json<Vec<String>>, AppError> {
    let user_id = auth_user.user_id_uuid()?;
    let permissions = state
        .permission_service
        .list_user_permissions(user_id, space_id)
        .await?;
    Ok(Json(permissions))
}

pub fn router() -> axum::Router<AppState> {
    use axum::routing::get;
    axum::Router::new().route("/spaces/{space_id}/my-permissions", get(get_my_permissions))
}
```

- [ ] **Step 2: Register the module in `apps/server/src/handlers/mod.rs`**

Add after the existing module declarations:
```rust
pub mod permissions;
```

- [ ] **Step 3: Register the route in `apps/server/src/main.rs`**

Add after the existing `.nest("/api/v1", ...)` calls (after line 157):
```rust
        .nest("/api/v1", rust_chat_server::handlers::permissions::router())
```

- [ ] **Step 4: Verify it compiles**

Run: `cd apps/server && cargo check`
Expected: Compiles without errors

---

## Task 3: Backend — Add channel WebSocket events + broadcast

**Files:**
- Modify: `apps/server/src/realtime/events.rs`
- Modify: `apps/server/src/services/channel_service.rs`

- [ ] **Step 1: Add channel events to `apps/server/src/realtime/events.rs`**

Add new variants to the `WsEvent` enum after the existing variants:

```rust
#[serde(rename = "channel.created")]
ChannelCreated(Channel),

#[serde(rename = "channel.updated")]
ChannelUpdated(Channel),

#[serde(rename = "channel.deleted")]
ChannelDeleted(Uuid),
```

Add import for the Channel type at the top:
```rust
use crate::domain::channel::Channel;
```

- [ ] **Step 2: Add Hub dependency to ChannelService**

Modify `apps/server/src/services/channel_service.rs`:

Add imports at the top:
```rust
use crate::realtime::events::WsEvent;
use crate::realtime::Hub;
use std::sync::Arc;
```

Change the struct to hold a Hub reference:
```rust
#[derive(Clone)]
pub struct ChannelService {
    repository: Arc<ChannelRepository>,
    hub: Arc<Hub>,
}
```

Update the constructor:
```rust
pub fn new(repository: Arc<ChannelRepository>, hub: Arc<Hub>) -> Self {
    Self { repository, hub }
}
```

- [ ] **Step 3: Update ChannelService instantiation in state.rs or main.rs**

Find where `ChannelService::new` is called (likely in `AppState::new` or main.rs). Pass the hub:
```rust
let channel_service = ChannelService::new(
    Arc::new(ChannelRepository::new(db.clone())),
    realtime_hub.clone(),
);
```

- [ ] **Step 4: Broadcast events from channel_service methods**

In `create_channel`, after creating the channel and before returning it, broadcast the event:
```rust
let _ = self
    .hub
    .publish_to_channel(
        space_id,
        WsEvent::ChannelCreated(channel.clone()).to_json(),
    )
    .await;
```

In `update_channel`, after updating, broadcast. Need to fetch the channel's space_id first (or have update_channel return a channel with space_id already set).

Let's change the approach: get the channel first, then update, then broadcast:
```rust
pub async fn update_channel(
    &self,
    channel_id: Uuid,
    input: UpdateChannel,
) -> Result<Channel, AppError> {
    let channel = self.repository.find_by_id(channel_id).await?;
    let space_id = channel.space_id;
    let updated = self
        .repository
        .update(channel_id, input.name, input.topic, input.visibility)
        .await?;
    let _ = self
        .hub
        .publish_to_channel(
            space_id,
            WsEvent::ChannelUpdated(updated.clone()).to_json(),
        )
        .await;
    Ok(updated)
}
```

In `delete_channel`, fetch the channel first to get space_id:
```rust
pub async fn delete_channel(&self, channel_id: Uuid) -> Result<(), AppError> {
    let channel = self.repository.find_by_id(channel_id).await?;
    let space_id = channel.space_id;
    self.repository.delete(channel_id).await?;
    let _ = self
        .hub
        .publish_to_channel(
            space_id,
            WsEvent::ChannelDeleted(channel_id).to_json(),
        )
        .await;
    Ok(())
}
```

- [ ] **Step 5: Verify it compiles**

Run: `cd apps/server && cargo check`
Expected: Compiles without errors

---

## Task 4: Frontend — Add `getMyPermissions` API + WS schema updates

**Files:**
- Modify: `apps/web/src/lib/api/channels.ts`
- Modify: `apps/web/src/lib/schemas/websocket.ts`

- [ ] **Step 1: Add `getMyPermissions` to `apps/web/src/lib/api/channels.ts`**

Add at the end of the file:

```typescript
export async function getMyPermissions(spaceId: string): Promise<string[]> {
  const token = getAccessToken();
  if (!token) {
    throw { status: 401, message: 'Not authenticated' } satisfies ApiError;
  }

  const response = await fetch(apiUrl(`/api/v1/spaces/${spaceId}/my-permissions`), {
    method: 'GET',
    headers: {
      'Content-Type': 'application/json',
      Authorization: `Bearer ${token}`,
    },
  });

  if (!response.ok) {
    const body = await response.json().catch(() => ({}));
    throw { status: response.status, message: body.message ?? 'Request failed' } satisfies ApiError;
  }

  const data: unknown = await response.json();
  return data as string[];
}
```

- [ ] **Step 2: Add `channel.deleted` event to `apps/web/src/lib/schemas/websocket.ts`**

Add to `WS_EVENT_TYPES` array (already has `channel.created`, `channel.updated` — add `channel.deleted` between them):
```typescript
'channel.deleted',
```

Add payload schema before the `payloadSchemaByType` mapping:
```typescript
export const channelDeletedPayloadSchema = z.object({
  channel_id: z.string().min(1),
});
export type ChannelDeletedPayload = z.infer<typeof channelDeletedPayloadSchema>;
```

Add to `EventPayloadMap`:
```typescript
'channel.deleted': ChannelDeletedPayload;
```

Add to `payloadSchemaByType`:
```typescript
'channel.deleted': channelDeletedPayloadSchema,
```

- [ ] **Step 3: Verify it compiles**

Run: `cd apps/web && npm run check`
Expected: No errors

---

## Task 5: Frontend — Space layout real-time sync + permissions

**Files:**
- Modify: `apps/web/src/routes/(app)/spaces/[spaceId]/+layout.svelte`

- [ ] **Step 1: Add imports and state**

In the script section, add imports:
```typescript
import { realtime } from '$lib/stores/realtime';
import { getMyPermissions } from '$lib/api/channels';
```

Add state variables:
```typescript
let permissions: string[] = $state([]);
```

- [ ] **Step 2: Connect to WebSocket and subscribe to events in onMount**

Update the `onMount` to connect to realtime and subscribe:

```typescript
onMount(() => {
  const token = getAccessToken();
  if (!token) {
    goto('/login');
    return;
  }

  realtime.connect(token);

  const unsubCreated = realtime.subscribe('channel.created', (payload) => {
    const ch = payload as Channel;
    if (ch.space_id === spaceId) {
      channels = [...channels, ch];
    }
  });

  const unsubUpdated = realtime.subscribe('channel.updated', (payload) => {
    const ch = payload as Channel;
    if (ch.space_id === spaceId) {
      channels = channels.map(c => c.id === ch.id ? ch : c);
    }
  });

  const unsubDeleted = realtime.subscribe('channel.deleted', (payload) => {
    const p = payload as { channel_id: string };
    channels = channels.filter(c => c.id !== p.channel_id);
  });

  load();

  return () => {
    unsubCreated();
    unsubUpdated();
    unsubDeleted();
  };
});
```

- [ ] **Step 3: Fetch permissions in `load()`**

Add to the `load()` function, after the existing channel/space fetch:
```typescript
const perms = await getMyPermissions(spaceId).catch(() => []);
permissions = perms;
```

- [ ] **Step 4: Pass permissions to ChannelList**

Update the ChannelList usage:
```svelte
<ChannelList {channels} {permissions} />
```

- [ ] **Step 5: Verify it compiles**

Run: `cd apps/web && npm run check`
Expected: No errors

---

## Task 6: Frontend — ChannelList permission controls + empty states

**Files:**
- Modify: `apps/web/src/lib/components/channels/ChannelList.svelte`

- [ ] **Step 1: Accept permissions prop and create derived state**

Update the props:
```typescript
let {
  channels = [],
  permissions = [],
}: {
  channels: Channel[];
  permissions?: string[];
} = $props();
```

Add derived state:
```typescript
let canManageChannels = $derived(permissions.includes('manage_channels'));
```

- [ ] **Step 2: Update empty states**

Replace the existing empty state block:
```svelte
{#if sorted.length === 0}
  <div class="flex flex-col items-center justify-center px-4 py-12 text-center">
    {#if canManageChannels}
      <p class="text-sm text-rc-500">No channels yet.</p>
      <p class="mt-1 text-xs text-rc-600">Create your first channel to get started.</p>
    {:else}
      <p class="text-sm text-rc-500">No visible channels</p>
      <p class="mt-1 text-xs text-rc-600">Ask an admin to grant you access.</p>
    {/if}
  </div>
{:else}
```

- [ ] **Step 3: Add "+" button for manage_channels permission**

At the bottom of the channel list nav (after the `{/each}` loop), add:
```svelte
{#if canManageChannels}
  <div class="px-2 pt-2">
    <a
      href="/spaces/{page.params.spaceId}/admin/channels"
      class="flex w-full items-center gap-2 rounded-lg px-3 py-2 text-sm text-rc-400 transition hover:bg-white/5 hover:text-rc-200"
    >
      <span class="text-lg leading-none">+</span>
      <span>Add Channel</span>
    </a>
  </div>
{/if}
```

- [ ] **Step 4: Verify it compiles**

Run: `cd apps/web && npm run check`
Expected: No errors

---

## Task 7: Frontend — MessageComposer + MessageList permission controls

**Files:**
- Modify: `apps/web/src/routes/(app)/spaces/[spaceId]/channels/[channelId]/+page.svelte`
- Modify: `apps/web/src/lib/components/chat/MessageComposer.svelte`
- Modify: `apps/web/src/lib/components/chat/MessageList.svelte`

- [ ] **Step 1: Fetch permissions in channel page independently**

In `+page.svelte`, add import and fetch permissions during load:

```typescript
import { getMyPermissions } from '$lib/api/channels';

let permissions: string[] = $state([]);
let canSendMessages = $derived(permissions.includes('send_messages'));
```

In the `load()` function, add after the existing channel/flags/messages fetch:
```typescript
const perms = await getMyPermissions(spaceId).catch(() => []);
permissions = perms;
```

- [ ] **Step 2: Update composerDisabled in channel page**

In the channel page, update the composerDisabled derived to also check SendMessages:
```typescript
let canSendMessages = $derived(permissions.includes('send_messages'));

let composerDisabled = $derived.by(() => {
  if (viewState !== 'loaded') return true;
  if (flags === null) return true;
  if (!flags.text_enabled) return true;
  if (channelKind !== 'Text') return true;
  if (sending) return true;
  if (!canSendMessages) return true;
  return false;
});

let composerDisabledReason = $derived.by(() => {
  if (viewState !== 'loaded') return 'Loading channel...';
  if (channelKind !== 'Text') return 'This channel does not support text messages.';
  if (flags && !flags.text_enabled) return 'Text messages are disabled in this channel.';
  if (!canSendMessages) return 'You do not have permission to send messages.';
  return '';
});
```

- [ ] **Step 3: Add readOnly mode to MessageList**

Update `MessageList.svelte` to add a `readOnly` prop:

```typescript
let {
  messages = [],
  loading = false,
  hasMore = false,
  loadingMore = false,
  readOnly = false,
  onloadMore,
}: {
  messages: Message[];
  loading?: boolean;
  hasMore?: boolean;
  loadingMore?: boolean;
  readOnly?: boolean;
  onloadMore?: () => void;
} = $props();
```

Add the read-only badge after the `{#if loading}` / `{:else if messages.length === 0}` block, at the top of the messages area:

```svelte
{#if readOnly}
  <div class="flex items-center justify-center gap-2 border-b border-white/10 bg-amber-500/5 px-4 py-2">
    <span class="text-xs font-medium text-amber-400">Read-only</span>
    <span class="text-xs text-amber-300/60">You don't have permission to send messages</span>
  </div>
{/if}
```

- [ ] **Step 4: Pass readOnly to MessageList from channel page**

In `+page.svelte`, update MessageList usage:
```svelte
<MessageList
  {messages}
  loading={false}
  {hasMore}
  {loadingMore}
  readOnly={!canSendMessages}
  onloadMore={loadMore}
/>
```

- [ ] **Step 5: Verify it compiles**

Run: `cd apps/web && npm run check`
Expected: No errors

---

## Task 8: Frontend — Error handling with retry

**Files:**
- Modify: `apps/web/src/routes/(app)/spaces/[spaceId]/+layout.svelte`

- [ ] **Step 1: Add retry helper and state**

In `+layout.svelte`, add retry state:
```typescript
let retryCount = $state(0);
const MAX_RETRIES = 3;
```

Add a retry helper function:
```typescript
async function loadWithRetry(): Promise<void> {
  retryCount = 0;
  await load();
}

async function load() {
  viewState = 'loading';
  for (let attempt = 0; attempt <= MAX_RETRIES; attempt++) {
    try {
      const [channelResult, spaceResult] = await Promise.all([
        listVisibleChannels(spaceId),
        getSpace(spaceId).catch(() => null),
      ]);
      channels = channelResult;
      spaceName = spaceResult?.name ?? 'Channels';
      
      const perms = await getMyPermissions(spaceId).catch(() => []);
      permissions = perms;
      
      viewState = 'loaded';
      return;
    } catch (err: unknown) {
      const e = err as { status?: number; message?: string };
      if (e.status === 401) {
        goto('/login');
        return;
      }
      if (e.status === 403) {
        viewState = 'forbidden';
        errorMessage = 'You do not have permission to access this space.';
        return;
      }
      if (e.status === 404) {
        viewState = 'notfound';
        return;
      }
      // Network/server error — retry with exponential backoff
      if (attempt < MAX_RETRIES) {
        retryCount = attempt + 1;
        const delay = Math.min(1000 * Math.pow(2, attempt), 8000);
        await new Promise(r => setTimeout(r, delay));
      } else {
        viewState = 'error';
        errorMessage = 'Something went wrong. Please try again.';
      }
    }
  }
}
```

- [ ] **Step 2: Update retry button to use loadWithRetry**

Replace the existing button:
```svelte
<button
  onclick={loadWithRetry}
  class="rounded-lg bg-brand-600 px-4 py-2 text-sm font-medium text-white transition hover:bg-brand-500 focus-visible:outline-2 focus-visible:outline-brand-400"
>
  Retry
</button>
```

- [ ] **Step 3: Verify it compiles**

Run: `cd apps/web && npm run check`
Expected: No errors

---

## Task 9: Verification

- [ ] **Step 1: Run backend checks**

```bash
cd apps/server
cargo fmt --check
cargo clippy -- -D warnings
cargo test
```

Expected: Formatting clean, clippy clean, all tests pass

- [ ] **Step 2: Run frontend checks**

```bash
cd apps/web
npm run check
npm run build
```

Expected: Both commands succeed

- [ ] **Step 3: Fix any issues found**

If any verification fails, fix the issues and re-run until clean.
