<script lang="ts">
  import type { Role } from '$lib/schemas/roles';
  import type { PermissionKey } from '$lib/schemas/permissions';
  import PermissionChecklist from './PermissionChecklist.svelte';

  let {
    spaceId,
    role = null,
  }: {
    spaceId: string;
    role?: Role | null;
  } = $props();

  let roleName = $state('');
  let selectedPermissions = $state<Set<string>>(new Set());
  let saving = $state(false);
  let error = $state<string | null>(null);

  $effect(() => {
    roleName = role?.name ?? '';
    selectedPermissions = new Set(role?.permissions ?? []);
  });

  const isEditing = $derived(role !== null);
  const isBackendAvailable = $derived(false);

  function handleToggle(key: PermissionKey) {
    const next = new Set(selectedPermissions);
    if (next.has(key)) {
      next.delete(key);
    } else {
      next.add(key);
    }
    selectedPermissions = next;
  }

  async function handleSave() {
    if (!isBackendAvailable) return;
    saving = true;
    error = null;
    try {
      // Backend call pending server endpoint implementation
    } catch (e) {
      error = e instanceof Error ? e.message : 'Failed to save role';
    } finally {
      saving = false;
    }
  }
</script>

<div class="space-y-6">
  <div class="space-y-2">
    <label for="role-name" class="text-sm font-medium text-rc-200">
      Role name
    </label>
    <input
      id="role-name"
      type="text"
      bind:value={roleName}
      placeholder="e.g. Moderator"
      class="w-full rounded-lg border border-white/10 bg-rc-900 px-4 py-2.5 text-sm text-white placeholder:text-rc-400 focus:border-brand-400 focus:outline-none focus:ring-1 focus:ring-brand-400"
      maxlength={64}
    />
  </div>

  <div class="space-y-2">
    <h3 class="text-sm font-medium text-rc-200">Permissions</h3>
    <PermissionChecklist selected={selectedPermissions} onToggle={handleToggle} />
  </div>

  {#if error}
    <div class="rounded-lg bg-red-500/10 px-4 py-3 text-sm text-red-300" role="alert">
      {error}
    </div>
  {/if}

  <div class="flex items-center justify-between border-t border-white/10 pt-4">
    <div class="text-sm text-rc-400">
      {isEditing ? 'Editing existing role' : 'Creating new role'} &mdash;
      <span class="text-amber-400">Backend not available</span>
    </div>
    <button
      onclick={handleSave}
      disabled={!isBackendAvailable || saving}
      class="inline-flex items-center gap-2 rounded-lg bg-brand-600 px-5 py-2.5 text-sm font-medium text-white transition hover:bg-brand-500 disabled:cursor-not-allowed disabled:opacity-50"
      title={!isBackendAvailable ? 'Role CRUD endpoints are not yet implemented on the server' : undefined}
    >
      {saving ? 'Saving...' : 'Save role'}
    </button>
  </div>
</div>
