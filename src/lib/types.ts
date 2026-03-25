export type Role = 'user' | 'assistant';

export interface Message {
	id: string;
	role: Role;
	content: string;
	timestamp: string;
}

export interface ChatThread {
	id: string;
	title: string;
	lastMessage: string;
	messages: Message[];
}
