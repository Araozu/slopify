<script lang="ts">
	import { page } from '$app/state';
	import { loginUser, registerUser } from '$lib/auth-client';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';

	type AuthMode = 'login' | 'register';

	let mode = $state<AuthMode>('register');
	let name = $state('');
	let email = $state('');
	let password = $state('');
	let errorMessage = $state('');
	let isSubmitting = $state(false);

	const redirectTo = $derived(page.url.searchParams.get('redirectTo') || '/');
	const title = $derived(mode === 'register' ? 'Create your Slopify login' : 'Sign back in');
	const subtitle = $derived(
		mode === 'register'
			? 'One user, one password, enough structure to stop cosplaying a ghost.'
			: 'Use the same email and password you registered with.'
	);
	const submitLabel = $derived(mode === 'register' ? 'Create account' : 'Log in');

	async function handleSubmit() {
		if (isSubmitting) {
			return;
		}

		errorMessage = '';
		isSubmitting = true;

		try {
			if (mode === 'register') {
				await registerUser({ name, email, password });
			} else {
				await loginUser({ email, password });
			}

			window.location.assign(redirectTo);
		} catch (error) {
			errorMessage = error instanceof Error ? error.message : 'Authentication failed.';
		} finally {
			isSubmitting = false;
		}
	}

	function handleFormSubmit(event: SubmitEvent) {
		event.preventDefault();
		void handleSubmit();
	}

	function toggleMode(nextMode: AuthMode) {
		errorMessage = '';
		mode = nextMode;
	}
</script>

<div
	class="flex min-h-screen items-center justify-center bg-[radial-gradient(circle_at_top,theme(colors.primary/.14),transparent_32%),linear-gradient(180deg,theme(colors.background),theme(colors.muted/.35))] px-6 py-12"
>
	<div
		class="grid w-full max-w-5xl overflow-hidden rounded-[2rem] border bg-card/90 shadow-2xl shadow-primary/10 backdrop-blur-xl lg:grid-cols-[1.1fr_0.9fr]"
	>
		<section
			class="relative hidden min-h-[620px] overflow-hidden border-r bg-muted/50 p-10 lg:flex lg:flex-col lg:justify-between"
		>
			<div class="space-y-6">
				<p class="text-[11px] font-black tracking-[0.35em] text-primary uppercase">Slopify</p>
				<div class="space-y-4">
					<h1 class="max-w-md text-4xl leading-tight font-semibold text-foreground">
						Real threads deserve a real owner.
					</h1>
					<p class="max-w-sm text-sm leading-6 text-muted-foreground">
						This is intentionally bare-bones: email, password, name, and a session cookie. Enough to
						attach state to a person before settings land later.
					</p>
				</div>
			</div>

			<div class="grid gap-4">
				<div class="rounded-3xl border bg-background/90 p-5 shadow-sm">
					<p class="text-xs font-black tracking-[0.28em] text-muted-foreground uppercase">
						Included
					</p>
					<p class="mt-3 text-sm text-foreground/80">Per-user threads and a persistent login.</p>
				</div>
				<div class="rounded-3xl border bg-background/80 p-5 shadow-sm">
					<p class="text-xs font-black tracking-[0.28em] text-muted-foreground uppercase">Next</p>
					<p class="mt-3 text-sm text-foreground/80">
						User settings can hang off the same account record later without reworking auth.
					</p>
				</div>
			</div>
		</section>

		<section class="flex min-h-[620px] items-center justify-center p-6 sm:p-10">
			<form class="w-full max-w-md space-y-6" onsubmit={handleFormSubmit}>
				<div class="space-y-3">
					<div class="inline-flex rounded-full border bg-muted/70 p-1">
						<button
							type="button"
							class={`rounded-full px-4 py-2 text-sm transition-colors ${mode === 'register' ? 'bg-background text-foreground shadow-sm' : 'text-muted-foreground'}`}
							onclick={() => toggleMode('register')}
						>
							Register
						</button>
						<button
							type="button"
							class={`rounded-full px-4 py-2 text-sm transition-colors ${mode === 'login' ? 'bg-background text-foreground shadow-sm' : 'text-muted-foreground'}`}
							onclick={() => toggleMode('login')}
						>
							Log in
						</button>
					</div>
					<div class="space-y-2">
						<h2 class="text-3xl font-semibold tracking-tight">{title}</h2>
						<p class="text-sm leading-6 text-muted-foreground">{subtitle}</p>
					</div>
				</div>

				<div class="space-y-4">
					{#if mode === 'register'}
						<div class="space-y-2">
							<label class="text-sm font-medium" for="name">Name</label>
							<Input id="name" bind:value={name} placeholder="Slop Queen" autocomplete="name" />
						</div>
					{/if}

					<div class="space-y-2">
						<label class="text-sm font-medium" for="email">Email</label>
						<Input
							id="email"
							bind:value={email}
							type="email"
							placeholder="you@slopify.ai"
							autocomplete="email"
						/>
					</div>

					<div class="space-y-2">
						<label class="text-sm font-medium" for="password">Password</label>
						<Input
							id="password"
							bind:value={password}
							type="password"
							placeholder="At least 8 characters"
							autocomplete={mode === 'register' ? 'new-password' : 'current-password'}
						/>
					</div>
				</div>

				{#if errorMessage}
					<p
						class="rounded-2xl border border-destructive/20 bg-destructive/10 px-4 py-3 text-sm text-destructive"
					>
						{errorMessage}
					</p>
				{/if}

				<Button type="submit" class="h-11 w-full rounded-2xl" disabled={isSubmitting}>
					{isSubmitting ? 'Working...' : submitLabel}
				</Button>
			</form>
		</section>
	</div>
</div>
