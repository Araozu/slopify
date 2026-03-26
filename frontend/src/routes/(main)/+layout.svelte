<script lang="ts">
	import { resolve } from '$app/paths';
	import { UserIcon, SignOutIcon, GearIcon, ClockCounterClockwiseIcon } from 'phosphor-svelte';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import * as Avatar from '$lib/components/ui/avatar';
	import ModeToggle from '$lib/components/mode-toggle/mode-toggle.svelte';
	import ThemeToggle from '$lib/components/theme-toggle/theme-toggle.svelte';
	import { onMount } from 'svelte';
	import { theme } from '$lib/stores/theme';

	let { children } = $props();

	onMount(() => {
		theme.init();
	});
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
				href={resolve('/')}
				class="rounded-md px-2 py-0.5 text-foreground/90 transition-colors hover:bg-foreground/10"
			>
				Chat
			</a>
			<a
				href={resolve('/history')}
				class="rounded-md px-2 py-0.5 text-foreground/90 transition-colors hover:bg-foreground/10"
			>
				History
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
						<Avatar.Image src="https://github.com/shadcn.png" alt="@user" />
						<Avatar.Fallback>
							<UserIcon size={14} />
						</Avatar.Fallback>
					</Avatar.Root>
				</DropdownMenu.Trigger>
				<DropdownMenu.Content align="end" class="w-56">
					<DropdownMenu.Label class="font-normal">
						<div class="flex flex-col space-y-1">
							<p class="text-sm leading-none font-medium">SoyDev Girl</p>
							<p class="text-xs leading-none text-muted-foreground">latte@slopify.ai</p>
						</div>
					</DropdownMenu.Label>
					<DropdownMenu.Separator />
					<DropdownMenu.Group>
						<DropdownMenu.Item>
							<UserIcon class="mr-2 h-4 w-4" />
							<span>Profile</span>
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
