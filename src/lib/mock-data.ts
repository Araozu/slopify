import type { ChatThread } from './types';

export const MOCK_CHATS: ChatThread[] = [
	{
		id: '1',
		title: 'The skincare routine',
		lastMessage: 'The double cleanse is key!',
		messages: [
			{
				id: 'm1',
				role: 'user',
				content: "I'm looking for a way to make my code as glowy as my skin. Any tips?",
				timestamp: '10:00 AM'
			},
			{
				id: 'm2',
				role: 'assistant',
				content:
					"Bestie, it's all about that *refactor serum*. Clean imports, type safety, and absolutely NO inline hex codes. That's how you get that natural glow! ✨",
				timestamp: '10:01 AM'
			},
			{
				id: 'm3',
				role: 'user',
				content: 'Omg, thank you! What about the sidebar though? It feels a bit dry.',
				timestamp: '10:02 AM'
			},
			{
				id: 'm4',
				role: 'assistant',
				content:
					"We're adding that glassmorphism hydration right now. Think backdrop-blur-xl and some juicy Phosphor icons. It's going to be a total look. 💅",
				timestamp: '10:03 AM'
			}
		]
	},
	{
		id: '2',
		title: 'TS vs JS debate',
		lastMessage: 'Types are just better for the soul.',
		messages: [
			{
				id: 'm5',
				role: 'user',
				content: 'Why should I even bother with TypeScript? JS is so much faster to write.',
				timestamp: '11:15 AM'
			},
			{
				id: 'm6',
				role: 'assistant',
				content:
					'Um, because catching errors at compile-time is basically the same as preventing a breakout before it even starts? 🧴 TypeScript is the ultimate sunscreen for your codebase, babe. No UV (Unpredicted Variables) allowed!',
				timestamp: '11:16 AM'
			}
		]
	},
	{
		id: '3',
		title: 'Latte art tips',
		lastMessage: 'Wait, is it oat milk or almond?',
		messages: [
			{
				id: 'm7',
				role: 'user',
				content: 'Help! My heart looks like a blob. What milk is best?',
				timestamp: '09:20 AM'
			},
			{
				id: 'm8',
				role: 'assistant',
				content:
					"Oat milk for the creamiest texture, hands down. It's the high-fidelity prototyper of milks. ☕✨ Also, watch your pour angle—it's like CSS positioning, everything has to be *just* right.",
				timestamp: '09:21 AM'
			}
		]
	}
];
