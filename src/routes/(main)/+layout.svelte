<script lang="ts">
	import { UserIcon, SignOutIcon, GearIcon, ClockCounterClockwiseIcon } from 'phosphor-svelte';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import * as Avatar from '$lib/components/ui/avatar';
	import Logo from '$lib/assets/favicon.svg';

	let { children } = $props();
</script>

<div class="flex min-h-screen flex-col bg-background text-foreground">
	<header
		class="sticky top-0 z-50 flex h-16 w-full items-center justify-between border-b bg-background/95 px-4 backdrop-blur supports-[backdrop-filter]:bg-background/60 md:px-6"
	>
		<nav class="flex items-center gap-6 text-sm font-medium">
			<a href="/" class="flex items-center gap-2 text-lg font-semibold md:text-base">
				<img src={Logo} alt="Slopify" class="h-6 w-6" />
				<span class="hidden sm:inline-block">Slopify</span>
			</a>
			<a
				href="/"
				class="flex items-center gap-2 text-muted-foreground transition-colors hover:text-foreground"
			>
				Chat
			</a>
			<a
				href="/history"
				class="flex items-center gap-2 text-muted-foreground transition-colors hover:text-foreground"
			>
				History
			</a>
		</nav>

		<div class="flex items-center gap-4">
			<DropdownMenu.Root>
				<DropdownMenu.Trigger
					class="relative flex h-9 w-9 shrink-0 items-center justify-center overflow-hidden rounded-full hover:bg-muted focus-visible:ring-1 focus-visible:ring-ring focus-visible:outline-none"
				>
					<Avatar.Root class="h-9 w-9">
						<Avatar.Image src="https://github.com/shadcn.png" alt="@user" />
						<Avatar.Fallback>
							<UserIcon size={20} />
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
