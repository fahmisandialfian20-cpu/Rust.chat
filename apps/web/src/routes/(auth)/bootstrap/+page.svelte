<script lang="ts">
  import { goto } from '$app/navigation';
  import { ArrowRight, LoaderCircle, AlertCircle } from 'lucide-svelte';
  import { BootstrapSchema } from '$lib/schemas/auth';
  import { bootstrapOwner } from '$lib/api/auth';
  import { setAuth } from '$lib/stores/auth.svelte';

  let username = $state('');
  let password = $state('');
  let errors = $state<Record<string, string>>({});
  let apiError = $state<string | null>(null);
  let isSubmitting = $state(false);
  let isConflictError = $state(false);

  async function handleSubmit(e: Event) {
    e.preventDefault();
    apiError = null;
    isConflictError = false;
    errors = {};

    const result = BootstrapSchema.safeParse({ username, password });

    if (!result.success) {
      const fieldErrors: Record<string, string> = {};
      for (const issue of result.error.issues) {
        const key = String(issue.path[0] ?? '');
        if (key && !fieldErrors[key]) {
          fieldErrors[key] = issue.message;
        }
      }
      errors = fieldErrors;
      return;
    }

    isSubmitting = true;

    try {
      const response = await bootstrapOwner(result.data);
      setAuth(response);
      goto('/lobby');
    } catch (err: unknown) {
      const error = err as { status?: number; message?: string };
      if (error.status === 409) {
        apiError = 'This instance already has an owner.';
        isConflictError = true;
      } else {
        apiError = 'Something went wrong. Please try again.';
      }
    } finally {
      isSubmitting = false;
    }
  }
</script>

<svelte:head>
  <title>Bootstrap Hoster - Rust.chat</title>
</svelte:head>

<div class="rounded-card border border-white/10 bg-white/[0.045] p-8 shadow-soft">
  <div class="mb-6 space-y-2">
    <h2 class="text-xl font-semibold text-white">Create Hoster account</h2>
    <p class="text-sm text-rc-300">
      This is the first account after deployment. You will be the Hoster with full control over the instance.
    </p>
  </div>

  <form onsubmit={handleSubmit} novalidate class="space-y-5">
    {#if apiError}
      <div role="alert" class="flex items-start gap-3 rounded-lg border border-red-500/30 bg-red-500/10 p-4 text-sm text-red-200">
        <AlertCircle class="mt-0.5 size-4 shrink-0" aria-hidden="true" />
        <div class="space-y-1">
          <p>{apiError}</p>
          {#if isConflictError}
            <a href="/login" class="underline hover:text-red-100">Go to login</a>
          {/if}
        </div>
      </div>
    {/if}

    <div class="space-y-2">
      <label for="username" class="text-sm font-medium text-rc-100">Username</label>
      <input
        id="username"
        type="text"
        autocomplete="username"
        bind:value={username}
        aria-invalid={!!errors.username}
        aria-describedby={errors.username ? 'username-error' : undefined}
        placeholder="hoster"
        class="w-full rounded-lg border bg-rc-950/50 px-4 py-2.5 text-sm text-white placeholder-rc-400 outline-none transition
          {errors.username
            ? 'border-red-500/50 focus:border-red-400 focus:ring-2 focus:ring-red-500/20'
            : 'border-white/10 focus:border-brand-300/50 focus:ring-2 focus:ring-brand-500/20'}"
      />
      {#if errors.username}
        <p id="username-error" class="text-xs text-red-300">{errors.username}</p>
      {/if}
    </div>

    <div class="space-y-2">
      <label for="password" class="text-sm font-medium text-rc-100">Password</label>
      <input
        id="password"
        type="password"
        autocomplete="new-password"
        bind:value={password}
        aria-invalid={!!errors.password}
        aria-describedby={errors.password ? 'password-error' : undefined}
        placeholder="At least 6 characters"
        class="w-full rounded-lg border bg-rc-950/50 px-4 py-2.5 text-sm text-white placeholder-rc-400 outline-none transition
          {errors.password
            ? 'border-red-500/50 focus:border-red-400 focus:ring-2 focus:ring-red-500/20'
            : 'border-white/10 focus:border-brand-300/50 focus:ring-2 focus:ring-brand-500/20'}"
      />
      {#if errors.password}
        <p id="password-error" class="text-xs text-red-300">{errors.password}</p>
      {/if}
    </div>

    <button
      type="submit"
      disabled={isSubmitting}
      class="inline-flex w-full items-center justify-center gap-2 rounded-lg bg-brand-500 px-5 py-2.5 text-sm font-semibold text-white transition
        hover:bg-brand-400 focus:outline-none focus:ring-2 focus:ring-brand-300 focus:ring-offset-2 focus:ring-offset-rc-950
        disabled:cursor-not-allowed disabled:opacity-60"
    >
      {#if isSubmitting}
        <LoaderCircle class="size-4 animate-spin" aria-hidden="true" />
        Creating...
      {:else}
        Create Hoster
        <ArrowRight class="size-4" aria-hidden="true" />
      {/if}
    </button>
  </form>
</div>
