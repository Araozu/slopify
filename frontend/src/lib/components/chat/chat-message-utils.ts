import type { Message } from '$lib/types';

export function formatMessageTimestamp(timestamp: string): string {
	const date = new Date(timestamp);
	if (Number.isNaN(date.getTime())) {
		return timestamp;
	}

	return date.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });
}

export function getMessageText(message: Message) {
	const text = (message.parts ?? [])
		.filter((part) => part.kind === 'text')
		.map((part) => part.text)
		.join('');
	return text || message.content || '';
}

export function getMessageReasoning(message: Message) {
	return (message.parts ?? [])
		.filter((part) => part.kind === 'reasoning')
		.map((part) => part.text)
		.join('');
}
