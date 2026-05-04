<script lang="ts">
  let {
    status = 'unknown',
    size = 'sm',
  }: {
    status?: 'online' | 'offline' | 'idle' | 'unknown';
    size?: 'sm' | 'md';
  } = $props();

  let dotClass = $derived.by(() => {
    const base = 'inline-block shrink-0 rounded-full';
    const s = size === 'sm' ? 'size-2' : 'size-3';
    const color = {
      online: 'bg-emerald-500',
      offline: 'bg-rc-600',
      idle: 'bg-amber-400',
      unknown: 'bg-rc-700',
    }[status];
    return `${base} ${s} ${color}`;
  });

  let label = $derived.by(() => {
    return {
      online: 'Online',
      offline: 'Offline',
      idle: 'Idle',
      unknown: 'Unknown',
    }[status];
  });
</script>

<span class={dotClass} aria-label={label} role="status" title={label}></span>
