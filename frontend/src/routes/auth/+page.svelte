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

<div class="flex min-h-[calc(100vh-2rem)] items-center justify-center bg-background p-6">
	<div class="w-full max-w-md space-y-8">
		<div class="flex flex-col items-center space-y-3 text-center">
			<div
				class="flex h-12 w-12 items-center justify-center rounded-2xl bg-primary/10 text-primary shadow-sm ring-1 ring-primary/20"
			>
				<p class="text-xs font-black tracking-widest uppercase">Slop</p>
			</div>
			<div class="space-y-1">
				<h1 class="text-2xl font-bold tracking-tight">{title}</h1>
				<p class="text-sm text-muted-foreground/60">{subtitle}</p>
			</div>
		</div>

		<div class="rounded-[2rem] border bg-card/50 p-8 shadow-sm backdrop-blur-md">
			<form class="space-y-6" onsubmit={handleFormSubmit}>
				<div class="flex justify-center">
					<div class="inline-flex rounded-full border bg-muted/50 p-1">
						<button
							type="button"
							class={`rounded-full px-4 py-1.5 text-xs font-bold tracking-wide transition-all ${mode === 'register' ? 'bg-background text-primary shadow-sm' : 'text-muted-foreground/60 hover:text-foreground'}`}
							onclick={() => toggleMode('register')}
						>
							Register
						</button>
						<button
							type="button"
							class={`rounded-full px-4 py-1.5 text-xs font-bold tracking-wide transition-all ${mode === 'login' ? 'bg-background text-primary shadow-sm' : 'text-muted-foreground/60 hover:text-foreground'}`}
							onclick={() => toggleMode('login')}
						>
							Log in
						</button>
					</div>
				</div>

				<div class="space-y-4">
					{#if mode === 'register'}
						<div class="space-y-2">
							<label
								class="text-[10px] font-black tracking-widest text-muted-foreground/60 uppercase"
								for="name">Name</label
							>
							<Input
								id="name"
								bind:value={name}
								placeholder="Slop Queen"
								autocomplete="name"
								class="h-11 rounded-2xl bg-background/50"
							/>
						</div>
					{/if}

					<div class="space-y-2">
						<label
							class="text-[10px] font-black tracking-widest text-muted-foreground/60 uppercase"
							for="email">Email</label
						>
						<Input
							id="email"
							bind:value={email}
							type="email"
							placeholder="you@slopify.ai"
							autocomplete="email"
							class="h-11 rounded-2xl bg-background/50"
						/>
					</div>

					<div class="space-y-2">
						<label
							class="text-[10px] font-black tracking-widest text-muted-foreground/60 uppercase"
							for="password">Password</label
						>
						<Input
							id="password"
							bind:value={password}
							type="password"
							placeholder="••••••••"
							autocomplete={mode === 'register' ? 'new-password' : 'current-password'}
							class="h-11 rounded-2xl bg-background/50"
						/>
					</div>
				</div>

				{#if errorMessage}
					<p
						class="rounded-2xl border border-destructive/20 bg-destructive/10 px-4 py-3 text-xs font-medium text-destructive"
					>
						{errorMessage}
					</p>
				{/if}

				<Button
					type="submit"
					class="h-11 w-full rounded-2xl shadow-lg shadow-primary/10 transition-transform active:scale-[0.98]"
					disabled={isSubmitting}
				>
					{isSubmitting ? 'Working...' : submitLabel}
				</Button>
			</form>
		</div>

		<p
			class="text-center text-[10px] font-medium tracking-widest text-muted-foreground/40 uppercase"
		>
			{mode === 'register' ? 'Real threads deserve a real owner.' : 'Welcome back to the slop.'}
		</p>
	</div>
</div>
