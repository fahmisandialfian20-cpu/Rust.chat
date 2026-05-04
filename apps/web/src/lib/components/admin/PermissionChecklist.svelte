<script lang="ts">
  import { PERMISSION_GROUPS } from '$lib/schemas/permissions';
  import type { PermissionKey } from '$lib/schemas/permissions';

  let {
    selected = new Set<string>(),
    onToggle = (_key: PermissionKey) => {},
  }: {
    selected?: Set<string>;
    onToggle?: (key: PermissionKey) => void;
  } = $props();
</script>

<div class="space-y-6" role="group" aria-label="Permission checklist">
  {#each PERMISSION_GROUPS as group}
    <fieldset class="space-y-2">
      <legend class="mb-2 text-xs font-semibold uppercase tracking-wider text-rc-300">
        {group.label}
      </legend>
      <div class="space-y-1">
        {#each group.keys as key}
          <label
            class="flex cursor-pointer items-center gap-3 rounded-lg px-3 py-2 transition hover:bg-white/5"
          >
            <input
              type="checkbox"
              checked={selected.has(key)}
              onchange={() => onToggle(key)}
              class="size-4 rounded border-white/20 bg-rc-800 text-brand-500 focus:ring-2 focus:ring-brand-400"
            />
            <span class="text-sm text-rc-200">{key}</span>
          </label>
        {/each}
      </div>
    </fieldset>
  {/each}
</div>
