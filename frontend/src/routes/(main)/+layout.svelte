<script lang="ts">
	import { createQuery } from '@tanstack/svelte-query';
	import { goto } from '$app/navigation';
	import { resolve } from '$app/paths';
	import { invalidateAll } from '$app/navigation';
	import {
		UserIcon,
		SignOutIcon,
		GearIcon,
		ClockCounterClockwiseIcon,
		KeyIcon
	} from 'phosphor-svelte';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import * as Avatar from '$lib/components/ui/avatar';
	import { logoutUser } from '$lib/auth-client';
	import { currentUserQueryOptions } from '$lib/queries/auth-query';
	import type { AuthUser } from '$lib/types';
	import ModeToggle from '$lib/components/mode-toggle/mode-toggle.svelte';
	import ThemeToggle from '$lib/components/theme-toggle/theme-toggle.svelte';
	import { onMount } from 'svelte';
	import { theme } from '$lib/stores/theme';
	import { showAssistantStreamingText } from '$lib/stores/streaming-preference';

	let { children, data } = $props<{
		children: () => unknown;
		data: { user: AuthUser };
	}>();

	const currentUserQuery = createQuery(() => ({
		...currentUserQueryOptions(),
		initialData: data.user
	}));
	const user = $derived((currentUserQuery.data ?? data.user) as AuthUser);

	onMount(() => {
		theme.init();
		showAssistantStreamingText.init();
	});

	async function handleLogout() {
		await logoutUser();
		await invalidateAll();
		await goto(resolve('/auth'), { replaceState: true });
	}
</script>

<div class="flex min-h-screen flex-col bg-background text-foreground">
	<header
		class="sticky top-0 z-50 flex h-8 w-full items-center justify-between border-b bg-background/50 px-3 backdrop-blur-xl supports-backdrop-filter:bg-background/40 md:px-4"
	>
		<nav class="flex items-center gap-4 text-sm font-semibold">
			<a href={resolve('/')} class="flex items-center gap-2 transition-opacity hover:opacity-80">
				<span class="inline-block font-bold">✨ Slopify ✨</span>
			</a>
			<a
				href={resolve('/(main)/profile')}
				class="hidden rounded-md px-2 py-0.5 text-foreground/90 transition-colors hover:bg-foreground/10 sm:block"
			>
				Profile
			</a>
			<a
				href={resolve('/(main)/settings/keys')}
				class="hidden rounded-md px-2 py-0.5 text-foreground/90 transition-colors hover:bg-foreground/10 sm:block"
			>
				Keys
			</a>
		</nav>

		<div class="flex items-center gap-3">
			<span class="flex items-center gap-2">
				<ThemeToggle />
				<ModeToggle />
			</span>
			<DropdownMenu.Root>
				<DropdownMenu.Trigger
					class="relative flex h-6 w-6 shrink-0 items-center justify-center overflow-hidden rounded-full hover:bg-muted focus-visible:ring-1 focus-visible:ring-ring focus-visible:outline-none"
				>
					<Avatar.Root class="h-6 w-6">
						<Avatar.Image src="https://github.com/Araozu.png" alt="@user" />
						<Avatar.Fallback>
							<UserIcon size={14} />
						</Avatar.Fallback>
					</Avatar.Root>
				</DropdownMenu.Trigger>
				<DropdownMenu.Content align="end" class="w-56">
					<DropdownMenu.Label class="font-normal">
						<div class="flex flex-col space-y-1">
							<p class="text-sm leading-none font-medium">{user.name}</p>
							<p class="text-xs leading-none text-muted-foreground">{user.email}</p>
						</div>
					</DropdownMenu.Label>
					<DropdownMenu.Separator />
					<DropdownMenu.Group>
						<DropdownMenu.Item onclick={() => goto(resolve('/(main)/profile'))}>
							<UserIcon class="mr-2 h-4 w-4" />
							<span>Profile</span>
						</DropdownMenu.Item>
						<DropdownMenu.Item onclick={() => goto(resolve('/(main)/settings/keys'))}>
							<KeyIcon class="mr-2 h-4 w-4" />
							<span>API Keys</span>
						</DropdownMenu.Item>
						<DropdownMenu.Item>
							<GearIcon class="mr-2 h-4 w-4" />
							<span>Settings</span>
						</DropdownMenu.Item>
						<DropdownMenu.Item>
							<ClockCounterClockwiseIcon class="mr-2 h-4 w-4" />
							<span>Activity</span>
						</DropdownMenu.Item>
					</DropdownMenu.Group>
					<DropdownMenu.Separator />
					<DropdownMenu.Item
						class="focus:text-destructive-foreground text-destructive focus:bg-destructive"
						onclick={handleLogout}
					>
						<SignOutIcon class="mr-2 h-4 w-4" />
						<span>Log out</span>
					</DropdownMenu.Item>
				</DropdownMenu.Content>
			</DropdownMenu.Root>
		</div>
	</header>

	<main class="flex-1">
		{@render children()}
	</main>
</div>
