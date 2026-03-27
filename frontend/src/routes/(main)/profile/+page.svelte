<script lang="ts">
	import { createQuery } from '@tanstack/svelte-query';
	import { page } from '$app/state';
	import { EnvelopeSimpleIcon, FingerprintIcon, UserCircleIcon } from 'phosphor-svelte';
	import { currentUserQueryOptions } from '$lib/queries/auth-query';
	import type { AuthUser } from '$lib/types';

	const currentUserQuery = createQuery(() => ({
		...currentUserQueryOptions(),
		initialData: page.data.user as AuthUser
	}));

	const user = $derived((currentUserQuery.data ?? page.data.user) as AuthUser);
	const initials = $derived(
		user.name
			.split(' ')
			.map((segment) => segment[0] ?? '')
			.join('')
			.slice(0, 2)
			.toUpperCase()
	);
	const queryError = $derived(
		currentUserQuery.error instanceof Error ? currentUserQuery.error.message : ''
	);
</script>

<div
	class="min-h-[calc(100vh-2rem)] bg-[radial-gradient(circle_at_top,theme(colors.primary/.12),transparent_30%),linear-gradient(180deg,theme(colors.background),theme(colors.muted/.3))] px-6 py-8 sm:px-8 lg:px-10"
>
	<div class="mx-auto flex w-full max-w-5xl flex-col gap-6">
		<section
			class="overflow-hidden rounded-[2rem] border bg-card/90 shadow-xl shadow-primary/5 backdrop-blur-xl"
		>
			<div class="grid gap-0 lg:grid-cols-[1.1fr_0.9fr]">
				<div class="border-b bg-muted/40 p-8 lg:border-r lg:border-b-0 lg:p-10">
					<p class="text-[11px] font-black tracking-[0.3em] text-primary uppercase">Profile</p>
					<h1 class="mt-4 max-w-md text-4xl leading-tight font-semibold tracking-tight">
						Your account, minus the drama.
					</h1>
					<p class="mt-4 max-w-lg text-sm leading-6 text-muted-foreground">
						This is the plain-data version for now: who you are, what email owns the account, and
						the stable user ID the backend hangs everything off.
					</p>
				</div>

				<div class="flex flex-col justify-between gap-6 p-8 lg:p-10">
					<div class="flex items-center gap-4">
						<div
							class="flex h-18 w-18 items-center justify-center rounded-[1.75rem] bg-primary text-2xl font-black text-primary-foreground shadow-lg shadow-primary/20"
						>
							{initials}
						</div>
						<div>
							<p class="text-xs font-black tracking-[0.24em] text-muted-foreground uppercase">
								Signed in as
							</p>
							<h2 class="mt-2 text-2xl font-semibold tracking-tight">{user.name}</h2>
							<p class="mt-1 text-sm text-muted-foreground">{user.email}</p>
						</div>
					</div>

					<div class="grid gap-3 sm:grid-cols-2">
						<div class="rounded-[1.5rem] border bg-background/80 p-4">
							<p class="text-[11px] font-black tracking-[0.24em] text-muted-foreground uppercase">
								Data mode
							</p>
							<p class="mt-3 text-sm text-foreground/80">
								Read only for now, so nothing here can smudge your account state.
							</p>
						</div>
						<div class="rounded-[1.5rem] border bg-background/80 p-4">
							<p class="text-[11px] font-black tracking-[0.24em] text-muted-foreground uppercase">
								Connected
							</p>
							<p class="mt-3 text-sm text-foreground/80">
								Your user record is the owner for threads, sessions, and OpenRouter keys.
							</p>
						</div>
					</div>
				</div>
			</div>
		</section>

		{#if queryError}
			<p
				class="rounded-2xl border border-destructive/20 bg-destructive/10 px-4 py-3 text-sm text-destructive"
			>
				{queryError}
			</p>
		{/if}

		<section class="grid gap-4 md:grid-cols-3">
			<div class="rounded-[1.75rem] border bg-card/90 p-5 shadow-sm">
				<div class="flex items-center gap-3">
					<div class="rounded-2xl bg-primary/10 p-2 text-primary">
						<UserCircleIcon size={18} weight="fill" />
					</div>
					<p class="text-[11px] font-black tracking-[0.24em] text-muted-foreground uppercase">
						Name
					</p>
				</div>
				<p class="mt-4 text-lg font-semibold tracking-tight">{user.name}</p>
			</div>

			<div class="rounded-[1.75rem] border bg-card/90 p-5 shadow-sm">
				<div class="flex items-center gap-3">
					<div class="rounded-2xl bg-primary/10 p-2 text-primary">
						<EnvelopeSimpleIcon size={18} weight="fill" />
					</div>
					<p class="text-[11px] font-black tracking-[0.24em] text-muted-foreground uppercase">
						Email
					</p>
				</div>
				<p class="mt-4 text-lg font-semibold tracking-tight break-all">{user.email}</p>
			</div>

			<div class="rounded-[1.75rem] border bg-card/90 p-5 shadow-sm">
				<div class="flex items-center gap-3">
					<div class="rounded-2xl bg-primary/10 p-2 text-primary">
						<FingerprintIcon size={18} weight="fill" />
					</div>
					<p class="text-[11px] font-black tracking-[0.24em] text-muted-foreground uppercase">
						User ID
					</p>
				</div>
				<p class="mt-4 font-mono text-sm break-all text-foreground/80">{user.id}</p>
			</div>
		</section>
	</div>
</div>
