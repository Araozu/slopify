<script lang="ts">
	import { HammerIcon } from 'phosphor-svelte';
	import { cn } from '$lib/utils';

	interface Props {
		collapsed: boolean;
	}

	let { collapsed }: Props = $props();
</script>

<aside
	id="stream-sidebar"
	class={cn(
		'min-h-0 w-60 flex-col border-l bg-muted/20 backdrop-blur-md',
		collapsed ? 'hidden' : 'hidden lg:flex'
	)}
	aria-hidden={collapsed}
>
	<div class="flex flex-1 flex-col items-center justify-center gap-3 p-6 text-center">
		<HammerIcon size={32} class="text-muted-foreground/40" weight="duotone" />
		<p class="text-[11px] leading-relaxed text-muted-foreground/50">
			Under construction — something glow-up worthy coming soon
		</p>
	</div>
</aside>
