<script lang="ts">
  import { PERMISSION_GROUPS } from '$lib/schemas/permissions';
</script>

<svelte:head>
  <title>Admin &mdash; Role Editor &mdash; Rust.chat</title>
</svelte:head>

<div class="mx-auto max-w-3xl space-y-8 px-6 py-8">
  <div class="space-y-2">
    <h1 class="text-2xl font-bold text-white">Role Editor</h1>
    <p class="text-sm text-rc-300">
      Manage space roles and their permission assignments.
    </p>
  </div>

  <div
    class="rounded-lg border border-amber-500/20 bg-amber-500/5 px-5 py-4 text-sm text-amber-300"
    role="alert"
  >
    <strong class="font-semibold">Backend not available:</strong>
    Role CRUD endpoints are not yet implemented on the server.
    The permission checklist below is a preview of the available permissions.
    Role persistence and assignment will be enabled once the server endpoints are ready.
  </div>

  <div class="rounded-xl border border-white/10 bg-rc-900/50 p-6">
    <h2 class="mb-1 text-lg font-semibold text-white">Available Permissions</h2>
    <p class="mb-6 text-sm text-rc-400">
      These are all permission keys defined by the server.
      Once backend role CRUD is implemented, you will be able to assign these
      to roles per space. Each permission grants a specific action within the
      platform.
    </p>
    <p class="mb-6 text-sm text-amber-400">
      Backend permission context required before controls become functional.
    </p>

    <div class="space-y-6">
      {#each PERMISSION_GROUPS as group}
        <section>
          <h3 class="mb-2 text-xs font-semibold uppercase tracking-wider text-rc-300">
            {group.label}
          </h3>
          <div class="grid grid-cols-1 gap-1 sm:grid-cols-2">
            {#each group.keys as key}
              <div class="flex items-center gap-3 rounded-lg px-3 py-2 text-sm text-rc-300">
                <span class="size-2 rounded-full bg-brand-500/40" aria-hidden="true"></span>
                <code class="font-mono text-xs">{key}</code>
              </div>
            {/each}
          </div>
        </section>
      {/each}
    </div>
  </div>

  <div class="rounded-xl border border-white/10 bg-rc-900/50 p-6">
    <h2 class="mb-1 text-lg font-semibold text-white">Backend Permission Context</h2>
    <p class="text-sm text-rc-400">
      The following values will be populated once the backend is available:
    </p>
    <ul class="mt-4 space-y-2 text-sm text-rc-400">
      <li class="flex items-center gap-2">
        <span class="size-1.5 rounded-full bg-rc-600" aria-hidden="true"></span>
        Space role list
      </li>
      <li class="flex items-center gap-2">
        <span class="size-1.5 rounded-full bg-rc-600" aria-hidden="true"></span>
        Current user permission context
      </li>
      <li class="flex items-center gap-2">
        <span class="size-1.5 rounded-full bg-rc-600" aria-hidden="true"></span>
        Role assignment editor
      </li>
      <li class="flex items-center gap-2">
        <span class="size-1.5 rounded-full bg-rc-600" aria-hidden="true"></span>
        Save and delete actions
      </li>
    </ul>
  </div>
</div>
