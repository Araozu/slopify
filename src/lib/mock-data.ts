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
			},
			{
				id: 'm9',
				role: 'user',
				content: 'Wait, is it oat milk or almond?',
				timestamp: '09:22 AM'
			},
			{
				id: 'm10',
				role: 'assistant',
				content:
					'OMG did you not read my message?? 😭 Oat milk queen, always. Almond is for sad protein shakes.',
				timestamp: '09:22 AM'
			}
		]
	},
	{
		id: '4',
		title: 'CSS naming wars',
		lastMessage: 'BEM is the only way, bestie.',
		messages: [
			{
				id: 'm11',
				role: 'user',
				content:
					'Why are my CSS class names so random? One day I use "red-button", next day "submit-btn" 😩',
				timestamp: '02:30 PM'
			},
			{
				id: 'm12',
				role: 'assistant',
				content:
					'That\'s giving major "mixed skincare routine" energy. We need structure! Try BEM naming—Block__Element--Modifier. It\'s like a 10-step skincare routine for your markup. 📋',
				timestamp: '02:31 PM'
			},
			{
				id: 'm13',
				role: 'user',
				content: 'BEM? Is that the one with the weird double underscores?',
				timestamp: '02:32 PM'
			},
			{
				id: 'm14',
				role: 'assistant',
				content:
					"Yes babe! `.card__header--highlighted` like that. It's a whole system. Once you start, you can't go back. It's the same as with serums—you commit or you don't. 💉",
				timestamp: '02:33 PM'
			},
			{
				id: 'm15',
				role: 'user',
				content: 'BEM is the only way, bestie.',
				timestamp: '02:34 PM'
			}
		]
	},
	{
		id: '5',
		title: 'State management drama',
		lastMessage: 'Zustand is just... prettier Redux.',
		messages: [
			{
				id: 'm16',
				role: 'user',
				content:
					"My app state is a MESS. It's like my skincare products—scattered everywhere, half empty, no organization.",
				timestamp: '03:15 PM'
			},
			{
				id: 'm17',
				role: 'assistant',
				content:
					"We need toMarie Kondo your state management. Does it spark joy? If not, maybe try Zustand. It's minimal, clean, and smells like fresh linen. 🧘‍♀️",
				timestamp: '03:16 PM'
			},
			{
				id: 'm18',
				role: 'user',
				content:
					'I tried Redux once. It was like opening 47 different serums to find the one I needed. 😩',
				timestamp: '03:17 PM'
			},
			{
				id: 'm19',
				role: 'assistant',
				content:
					"Redux is for people who love paperwork. Zustand? It's those satisfying sheet masks—simple, effective, single-use (kinda). Try it!",
				timestamp: '03:18 PM'
			},
			{
				id: 'm20',
				role: 'user',
				content: 'Zustand is just... prettier Redux.',
				timestamp: '03:19 PM'
			},
			{
				id: 'm21',
				role: 'assistant',
				content: 'And PRETTIER is all that matters sometimes, bestie. ✨',
				timestamp: '03:19 PM'
			}
		]
	},
	{
		id: '6',
		title: 'Git commit struggles',
		lastMessage: 'feat: add coffee dependency',
		messages: [
			{
				id: 'm22',
				role: 'user',
				content: 'How do I write good commit messages? Mine all say "fixed stuff" 😂',
				timestamp: '04:00 PM'
			},
			{
				id: 'm23',
				role: 'assistant',
				content:
					"Conventional commits bestie! It's like categorizing your skincare by morning vs night. `feat:`, `fix:`, `docs:`—everything has its place! 📝",
				timestamp: '04:01 PM'
			},
			{
				id: 'm24',
				role: 'user',
				content: 'So like "fix: moisturize under eyes"?',
				timestamp: '04:02 PM'
			},
			{
				id: 'm25',
				role: 'assistant',
				content:
					'I\'m screaming 😂 Yes exactly! Make it descriptive. "fix: resolve hydration gap in sidebar" not "update stuff".',
				timestamp: '04:03 PM'
			},
			{
				id: 'm26',
				role: 'user',
				content: 'My last commit was literally just "oops"',
				timestamp: '04:04 PM'
			},
			{
				id: 'm27',
				role: 'assistant',
				content:
					"We've all been there babe. It's the skincare equivalent of using hand cream as hair serum. Mistake? Yes. End of the world? No. Just amend it! 💅",
				timestamp: '04:05 PM'
			},
			{
				id: 'm28',
				role: 'user',
				content: 'feat: add coffee dependency',
				timestamp: '04:06 PM'
			}
		]
	},
	{
		id: '7',
		title: 'API design struggles',
		lastMessage: 'REST is classic, but have you tried gRPC?',
		messages: [
			{
				id: 'm29',
				role: 'user',
				content: 'Should my API endpoints return arrays or paginated objects?',
				timestamp: '05:00 PM'
			},
			{
				id: 'm30',
				role: 'assistant',
				content:
					'Paginated objects babe! Returning raw arrays is like dumping your whole makeup collection on the floor. Pagination is that cute vanity organizer. 🎀',
				timestamp: '05:01 PM'
			},
			{
				id: 'm31',
				role: 'user',
				content: 'So like { data: [], total: 100, page: 1 }?',
				timestamp: '05:02 PM'
			},
			{
				id: 'm32',
				role: 'assistant',
				content:
					"Exactly! That's the skincare routine of APIs. Proper cleansing, toning, moisturizing. Each step in its place.",
				timestamp: '05:03 PM'
			},
			{
				id: 'm33',
				role: 'user',
				content: "What about GraphQL? Everyone says it's the holy grail",
				timestamp: '05:04 PM'
			},
			{
				id: 'm34',
				role: 'assistant',
				content:
					"GraphQL is amazing but it's like that 15-step Korean skincare routine. Powerful? Yes. Necessary for everyone? Maybe not. Start simple, optimize later. 💆‍♀️",
				timestamp: '05:05 PM'
			},
			{
				id: 'm35',
				role: 'user',
				content: 'REST is classic, but have you tried gRPC?',
				timestamp: '05:06 PM'
			}
		]
	},
	{
		id: '8',
		title: 'Debugging at midnight',
		lastMessage: 'Have you tried turning it off and on again?',
		messages: [
			{
				id: 'm36',
				role: 'user',
				content: "Why does my code only break at 2am? It's giving haunted mansion energy 👻",
				timestamp: '02:00 AM'
			},
			{
				id: 'm37',
				role: 'assistant',
				content:
					"Because the code knows you're tired, babe. It strikes when you're vulnerable. Always the villain arc in horror movies! 🎬",
				timestamp: '02:01 AM'
			},
			{
				id: 'm38',
				role: 'user',
				content: "I've been staring at this for 4 hours. send help.",
				timestamp: '02:15 AM'
			},
			{
				id: 'm39',
				role: 'assistant',
				content:
					'Step away. Get some water. Maybe that 47-step nighttime routine? The bug will still be there tomorrow but your sanity needs hydrating too. 💧',
				timestamp: '02:16 AM'
			},
			{
				id: 'm40',
				role: 'user',
				content: 'I found it. It was a missing semicolon.',
				timestamp: '02:45 AM'
			},
			{
				id: 'm41',
				role: 'assistant',
				content:
					"A SEMICOLON?? That's the equivalent of forgetting sunscreen and getting burned. Take it as a lesson—always protect your code with proper linting! ☀️",
				timestamp: '02:46 AM'
			},
			{
				id: 'm42',
				role: 'user',
				content: 'Have you tried turning it off and on again?',
				timestamp: '02:47 AM'
			}
		]
	},
	{
		id: '9',
		title: 'React vs Vue debate',
		lastMessage: 'At the end of the day, they all compile to JS anyway.',
		messages: [
			{
				id: 'm43',
				role: 'user',
				content: 'Should I learn React or Vue? Everyone has opinions 😭',
				timestamp: '01:00 PM'
			},
			{
				id: 'm44',
				role: 'assistant',
				content:
					"It's like foundation shades—React is the classic MAC, Vue is that viral rare beauty tint. Both give glow, just different vibes. 💄",
				timestamp: '01:01 PM'
			},
			{
				id: 'm45',
				role: 'user',
				content: 'So Vue is easier?',
				timestamp: '01:02 PM'
			},
			{
				id: 'm46',
				role: 'assistant',
				content:
					'Vue has a gentler learning curve, like starter skincare kits. React is more like jumping into medical-grade actives. Both can give amazing results if you commit!',
				timestamp: '01:03 PM'
			},
			{
				id: 'm47',
				role: 'user',
				content: "Angular though? Someone told me it's dead?",
				timestamp: '01:04 PM'
			},
			{
				id: 'm48',
				role: 'assistant',
				content:
					"Angular is NOT dead bestie, it's just... high maintenance. Like those LED face masks. Expensive, complicated, but the results?? *chef's kiss* 😘",
				timestamp: '01:05 PM'
			},
			{
				id: 'm49',
				role: 'user',
				content: 'At the end of the day, they all compile to JS anyway.',
				timestamp: '01:06 PM'
			}
		]
	},
	{
		id: '10',
		title: 'Package.json mysteries',
		lastMessage: 'devDependencies vs dependencies... help?',
		messages: [
			{
				id: 'm50',
				role: 'user',
				content: "What's the difference between dependencies and devDependencies?",
				timestamp: '11:30 AM'
			},
			{
				id: 'm51',
				role: 'assistant',
				content:
					"OK think of it like this—dependencies are your daily serums, they go in the actual product. devDependencies are your skincare tools, they help you apply things but don't go ON your face. 🧴",
				timestamp: '11:31 AM'
			},
			{
				id: 'm52',
				role: 'user',
				content: 'So React is a dependency, but TypeScript is dev?',
				timestamp: '11:32 AM'
			},
			{
				id: 'm53',
				role: 'assistant',
				content:
					'Yes babe! React runs in the browser (= your face). TypeScript compiles away (= your fancy jade roller, not part of the final look).',
				timestamp: '11:33 AM'
			},
			{
				id: 'm54',
				role: 'user',
				content: 'What about webpack? I have no idea what that is',
				timestamp: '11:34 AM'
			},
			{
				id: 'm55',
				role: 'assistant',
				content:
					'Webpack is like your makeup brushes—essential for applying everything correctly, but nobody sees them in the final look. It bundles your code up all pretty. 🖌️',
				timestamp: '11:35 AM'
			},
			{
				id: 'm56',
				role: 'user',
				content: 'devDependencies vs dependencies... help?',
				timestamp: '11:36 AM'
			}
		]
	}
];
