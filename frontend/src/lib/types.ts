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
	system_prompt?: string | null;
	timestamp: string;
}

export interface Tag {
	id: string;
	name: string;
	color: string;
}

export interface Thread {
	id: string;
	title: string;
	model?: string;
	tags?: Tag[];
}

export interface ChatThread extends Thread {
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

export interface OpenRouterModel {
	id: string;
	modelId: string;
}

export interface CopilotModel {
	id: string;
	modelId: string;
}

export interface CopilotToken {
	id: string;
	name: string;
	githubToken: string;
}

export interface ZenApiKey {
	id: string;
	name: string;
	apiKey: string;
}

export type ProviderSlug = 'openrouter' | 'github-copilot' | 'opencode-zen';

export interface ProviderCredential {
	id: string;
	name: string;
	provider: ProviderSlug;
	token: string;
}
