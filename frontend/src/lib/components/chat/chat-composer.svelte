<script lang="ts">
	import { PaperPlaneRightIcon } from 'phosphor-svelte';
	import { Button } from '$lib/components/ui/button';
	import { Textarea } from '$lib/components/ui/textarea';
	import type { ProviderCredential, Thread } from '$lib/types';

	interface Props {
		draft?: string;
		model?: string;
		selectedCredential: ProviderCredential | null;
		isSending: boolean;
		isBootstrapping: boolean;
		activeThread: Thread | null;
		onSend: () => void;
		onComposerKeydown: (event: KeyboardEvent) => void;
	}

	let {
		draft = $bindable(''),
		model = $bindable(''),
		selectedCredential,
		isSending,
		isBootstrapping,
		activeThread,
		onSend,
		onComposerKeydown
	}: Props = $props();
</script>

<footer class="p-4 md:p-6">
	<div
		class="mx-auto flex max-w-3xl items-center gap-3 rounded-xl bg-muted/40 p-2.5 shadow-inner ring-1 ring-border/50 transition-all focus-within:bg-background/60 focus-within:ring-primary/30"
	>
		<Textarea
			bind:value={draft}
			placeholder="Message Slopify..."
			class="dark:bg-initial bg-initial field-sizing-content max-h-[calc(7*1.5rem+1rem)] min-h-[2.25rem] resize-none border-0 px-3 py-2 text-sm leading-6 placeholder:text-muted-foreground/40 focus-visible:ring-0 focus-visible:ring-offset-0"
			disabled={isSending || isBootstrapping || !activeThread}
			onkeydown={onComposerKeydown}
			rows={1}
		/>
		<div class="relative shrink-0">
			<Button
				size="icon-sm"
				variant="default"
				class="h-9 w-9 rounded-lg shadow-lg shadow-primary/20 transition-transform active:scale-95"
				disabled={isSending ||
					isBootstrapping ||
					!activeThread ||
					!draft.trim() ||
					!selectedCredential ||
					!model.trim()}
				onclick={onSend}
			>
				<PaperPlaneRightIcon size={18} weight="bold" />
			</Button>
			<span
				class="pointer-events-none absolute top-1/2 left-[calc(100%+0.6rem)] -translate-y-1/2 text-[9px] font-medium tracking-widest whitespace-nowrap text-muted-foreground/25 uppercase"
			>
				double-check responses
			</span>
		</div>
	</div>
</footer>
