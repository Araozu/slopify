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

<div class="flex h-[calc(100vh-2rem)] min-h-0 w-full flex-col bg-background">
	<header class="flex items-center border-b bg-background/20 px-6 py-1 backdrop-blur-xl">
		<div class="flex items-center gap-2">
			<div class="flex h-6 w-6 items-center justify-center rounded-full bg-primary/10 text-primary">
				<UserCircleIcon size={14} weight="fill" />
			</div>
			<h1 class="text-sm font-bold tracking-tight">Profile</h1>
		</div>
	</header>

	<div class="flex-1 overflow-y-auto">
		<div class="mx-auto w-full max-w-3xl space-y-12 px-6 py-10">
			<section class="space-y-6">
				<div class="flex items-center gap-6">
					<div
						class="flex h-20 w-20 items-center justify-center rounded-[2rem] bg-primary text-3xl font-black text-primary-foreground shadow-lg shadow-primary/20"
					>
						{initials}
					</div>
					<div class="space-y-1">
						<p class="text-[10px] font-black tracking-[0.2em] text-muted-foreground/50 uppercase">
							Account Owner
						</p>
						<h2 class="text-3xl font-bold tracking-tight">{user.name}</h2>
						<p class="text-sm text-muted-foreground/60">{user.email}</p>
					</div>
				</div>

				<div class="grid gap-4 sm:grid-cols-2">
					<div class="rounded-3xl border bg-muted/30 p-5 shadow-inner ring-1 ring-border/50">
						<p class="text-[10px] font-black tracking-widest text-muted-foreground/40 uppercase">
							Data mode
						</p>
						<p class="mt-2 text-xs leading-relaxed text-foreground/60">
							Read only for now. Your account state is locked to prevent accidental smudging.
						</p>
					</div>
					<div class="rounded-3xl border bg-muted/30 p-5 shadow-inner ring-1 ring-border/50">
						<p class="text-[10px] font-black tracking-widest text-muted-foreground/40 uppercase">
							Connected
						</p>
						<p class="mt-2 text-xs leading-relaxed text-foreground/60">
							Your record owns all threads, sessions, and provider keys.
						</p>
					</div>
				</div>
			</section>

			{#if queryError}
				<p
					class="rounded-2xl border border-destructive/20 bg-destructive/10 px-4 py-3 text-xs font-medium text-destructive"
				>
					{queryError}
				</p>
			{/if}

			<section class="space-y-4">
				<h3 class="text-[10px] font-black tracking-widest text-muted-foreground/40 uppercase">
					Identity Details
				</h3>
				<div class="grid gap-3 md:grid-cols-3">
					<div class="rounded-2xl border bg-card/50 p-4 shadow-sm backdrop-blur-sm">
						<div class="flex items-center gap-2 text-primary/60">
							<UserCircleIcon size={16} weight="bold" />
							<span class="text-[10px] font-black tracking-widest uppercase">Name</span>
						</div>
						<p class="mt-3 text-sm font-bold tracking-tight">{user.name}</p>
					</div>

					<div class="rounded-2xl border bg-card/50 p-4 shadow-sm backdrop-blur-sm">
						<div class="flex items-center gap-2 text-primary/60">
							<EnvelopeSimpleIcon size={16} weight="bold" />
							<span class="text-[10px] font-black tracking-widest uppercase">Email</span>
						</div>
						<p class="mt-3 truncate text-sm font-bold tracking-tight">{user.email}</p>
					</div>

					<div class="rounded-2xl border bg-card/50 p-4 shadow-sm backdrop-blur-sm">
						<div class="flex items-center gap-2 text-primary/60">
							<FingerprintIcon size={16} weight="bold" />
							<span class="text-[10px] font-black tracking-widest uppercase">User ID</span>
						</div>
						<p class="mt-3 font-mono text-[10px] text-muted-foreground/70">{user.id}</p>
					</div>
				</div>
			</section>

			<footer class="pt-8">
				<p
					class="text-center text-[10px] font-medium tracking-widest text-muted-foreground/30 uppercase"
				>
					Slopify identity system v1.0 • Stable and boring.
				</p>
			</footer>
		</div>
	</div>
</div>
