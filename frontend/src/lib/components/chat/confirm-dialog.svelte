<script lang="ts">
	import { Dialog } from 'bits-ui';
	import { Button } from '$lib/components/ui/button';

	interface Props {
		open?: boolean;
		title: string;
		description: string;
		confirmLabel?: string;
		onConfirm: () => void;
		onCancel: () => void;
	}

	let {
		open = $bindable(false),
		title,
		description,
		confirmLabel = 'Delete',
		onConfirm,
		onCancel
	}: Props = $props();
</script>

<Dialog.Root bind:open onOpenChange={(v) => !v && onCancel()}>
	<Dialog.Portal>
		<Dialog.Overlay
			class="fixed inset-0 z-50 bg-background/80 backdrop-blur-sm data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:animate-in data-[state=open]:fade-in-0"
		/>
		<Dialog.Content
			class="fixed top-1/2 left-1/2 z-50 w-full max-w-md -translate-x-1/2 -translate-y-1/2 rounded-xl border bg-background p-6 shadow-xl outline-none data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=closed]:zoom-out-95 data-[state=open]:animate-in data-[state=open]:fade-in-0 data-[state=open]:zoom-in-95"
		>
			<Dialog.Title class="text-sm font-semibold">{title}</Dialog.Title>
			<Dialog.Description class="mt-2 text-sm text-muted-foreground"
				>{description}</Dialog.Description
			>
			<div class="mt-6 flex justify-end gap-2">
				<Button variant="outline" size="sm" onclick={onCancel}>Cancel</Button>
				<Button variant="destructive" size="sm" onclick={onConfirm}>{confirmLabel}</Button>
			</div>
		</Dialog.Content>
	</Dialog.Portal>
</Dialog.Root>
