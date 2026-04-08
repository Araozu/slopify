export type Role = 'user' | 'assistant';
export type MessageStatus = 'streaming' | 'completed' | 'failed';
export type UserMessageDeliveryStatus = 'sent' | 'delivered';

export type MessagePart = { kind: 'text'; text: string } | { kind: 'reasoning'; text: string };

export interface Message {
	id: string;
	role: Role;
	status?: MessageStatus;
	deliveryStatus?: UserMessageDeliveryStatus;
	parts?: MessagePart[];
	provider?: {
		provider: string;
		model: string;
		endpoint?: string | null;
	};
	metadata?: {
		finish_reason?: string | null;
		vendor_metadata?: unknown;
	};
	content: string;
	timestamp: string;
}

export interface Thread {
	id: string;
	title: string;
	model?: string;
}

export interface ChatThread extends Thread {
	lastMessage: string;
	messages: Message[];
}

export interface AuthUser {
	id: string;
	email: string;
	name: string;
}

export interface OpenRouterApiKey {
	id: string;
	name: string;
	apiKey: string;
}

export interface SystemPrompt {
	id: string;
	name: string;
	content: string;
}
