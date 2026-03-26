export type Role = 'user' | 'assistant';

export interface Message {
	id: string;
	role: Role;
	content: string;
	timestamp: string;
}

export interface Thread {
	id: string;
	title: string;
}

export interface ChatThread extends Thread {
	lastMessage: string;
	messages: Message[];
}
