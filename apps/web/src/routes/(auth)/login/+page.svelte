<script lang="ts">
  import { goto } from '$app/navigation';
  import { ArrowRight, LoaderCircle, AlertCircle } from 'lucide-svelte';
  import { LoginSchema } from '$lib/schemas/auth';
  import { login } from '$lib/api/auth';
  import { setAuth } from '$lib/stores/auth.svelte';
  import { currentClientPlatform } from '$lib/config';

  let username_or_email = $state('');
  let password = $state('');
  let errors = $state<Record<string, string>>({});
  let apiError = $state<string | null>(null);
  let isSubmitting = $state(false);

  async function handleSubmit(e: Event) {
    e.preventDefault();
    apiError = null;
    errors = {};

    const result = LoginSchema.safeParse({ username_or_email, password });

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
      const response = await login({
        ...result.data,
        client_metadata: { client_type: 'web', platform: currentClientPlatform() },
      });
      setAuth(response);
      goto('/lobby');
    } catch (err: unknown) {
      const error = err as { status?: number; message?: string };
      if (error.status === 401) {
        apiError = 'Invalid username or password.';
      } else {
        apiError = 'Something went wrong. Please try again.';
      }
    } finally {
      isSubmitting = false;
    }
  }
</script>

<svelte:head>
  <title>Login - Rust.chat</title>
</svelte:head>

<div class="rounded-card border border-white/10 bg-white/[0.045] p-8 shadow-soft">
  <div class="mb-6 space-y-2">
    <h2 class="text-xl font-semibold text-white">Welcome back</h2>
    <p class="text-sm text-rc-300">Sign in to your account to continue.</p>
  </div>

  <form onsubmit={handleSubmit} novalidate class="space-y-5">
    {#if apiError}
      <div role="alert" class="flex items-start gap-3 rounded-lg border border-red-500/30 bg-red-500/10 p-4 text-sm text-red-200">
        <AlertCircle class="mt-0.5 size-4 shrink-0" aria-hidden="true" />
        <p>{apiError}</p>
      </div>
    {/if}

    <div class="space-y-2">
      <label for="username_or_email" class="text-sm font-medium text-rc-100">Username or email</label>
      <input
        id="username_or_email"
        type="text"
        autocomplete="username"
        bind:value={username_or_email}
        aria-invalid={!!errors.username_or_email}
        aria-describedby={errors.username_or_email ? 'username-or-email-error' : undefined}
        placeholder="username or email"
        class="w-full rounded-lg border bg-rc-950/50 px-4 py-2.5 text-sm text-white placeholder-rc-400 outline-none transition
          {errors.username_or_email
            ? 'border-red-500/50 focus:border-red-400 focus:ring-2 focus:ring-red-500/20'
            : 'border-white/10 focus:border-brand-300/50 focus:ring-2 focus:ring-brand-500/20'}"
      />
      {#if errors.username_or_email}
        <p id="username-or-email-error" class="text-xs text-red-300">{errors.username_or_email}</p>
      {/if}
    </div>

    <div class="space-y-2">
      <label for="password" class="text-sm font-medium text-rc-100">Password</label>
      <input
        id="password"
        type="password"
        autocomplete="current-password"
        bind:value={password}
        aria-invalid={!!errors.password}
        aria-describedby={errors.password ? 'password-error' : undefined}
        placeholder="Password"
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
        Signing in...
      {:else}
        Sign in
        <ArrowRight class="size-4" aria-hidden="true" />
      {/if}
    </button>
  </form>

  <p class="mt-6 text-center text-sm text-rc-400">
    Don't have an account?
    <a href="/register" class="font-medium text-brand-300 hover:text-brand-200">Create one</a>
  </p>
  <p class="mt-2 text-center text-sm text-rc-400">
    First time deploying?
    <a href="/bootstrap" class="font-medium text-brand-300 hover:text-brand-200">Bootstrap Hoster</a>
  </p>
</div>
