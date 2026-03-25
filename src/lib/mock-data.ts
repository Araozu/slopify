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
	},
	{
		id: '11',
		title: 'The 10-step skincare journey',
		lastMessage: 'Retinol is not for the faint of heart.',
		messages: [
			{
				id: 'm57',
				role: 'user',
				content: 'I want to upgrade my skincare routine. Currently just using soap on my face 😭',
				timestamp: '08:00 AM'
			},
			{
				id: 'm58',
				role: 'assistant',
				content:
					"SOAP?? Bestie that's like using dish soap to wash your hair. We need to talk. Immediately. 🚨",
				timestamp: '08:01 AM'
			},
			{
				id: 'm59',
				role: 'user',
				content: 'It gets the job done tho??',
				timestamp: '08:02 AM'
			},
			{
				id: 'm60',
				role: 'assistant',
				content:
					"It's stripping ALL your natural oils babe. Your skin is screaming for help. Let's build you a proper routine step by step, OK? 💆‍♀️",
				timestamp: '08:03 AM'
			},
			{
				id: 'm61',
				role: 'user',
				content: "OK I'm listening...",
				timestamp: '08:05 AM'
			},
			{
				id: 'm62',
				role: 'assistant',
				content:
					'Step 1: Oil cleanser. Yes OIL. It dissolves all the gunk and makeup. Think of it like demelanizing your face.',
				timestamp: '08:06 AM'
			},
			{
				id: 'm63',
				role: 'user',
				content: "Won't that make me more oily??",
				timestamp: '08:07 AM'
			},
			{
				id: 'm64',
				role: 'assistant',
				content:
					"Opposite effect bestie! It balanced your skin's moisture barrier. Like how hydrating your code actually prevents more bugs. Counterintuitive but TRUE. ✨",
				timestamp: '08:08 AM'
			},
			{
				id: 'm65',
				role: 'user',
				content: "What's step 2?",
				timestamp: '08:10 AM'
			},
			{
				id: 'm66',
				role: 'assistant',
				content:
					"Water-based cleanser! Double cleanse is the move. First oil, then water-based. It's like clearing your cache twice. Nothing gets stuck. 🧹",
				timestamp: '08:11 AM'
			},
			{
				id: 'm67',
				role: 'user',
				content: "I've heard about toners? Do I need one?",
				timestamp: '08:15 AM'
			},
			{
				id: 'm68',
				role: 'assistant',
				content:
					"YES. Toner is like the primer of skincare. It preps your skin to absorb all the good stuff coming next. Don't skip it!",
				timestamp: '08:16 AM'
			},
			{
				id: 'm69',
				role: 'user',
				content: 'What about serums? There are so many options??',
				timestamp: '08:20 AM'
			},
			{
				id: 'm70',
				role: 'assistant',
				content:
					"OK so serums are the active ingredients. Vitamin C for brightening, hyaluronic acid for hydration, niacinamide for pores. It's like picking the right utility function. Choose ONE to start.",
				timestamp: '08:21 AM'
			},
			{
				id: 'm71',
				role: 'user',
				content: 'I have melasma spots. What should I do?',
				timestamp: '08:25 AM'
			},
			{
				id: 'm72',
				role: 'assistant',
				content:
					'Vitamin C serum is your bestie for that! Also SPF 50 like your life depends on it. Sun exposure makes melasma SO much worse. ☀️',
				timestamp: '08:26 AM'
			},
			{
				id: 'm73',
				role: 'user',
				content: 'Speaking of SPF, any recommendations?',
				timestamp: '08:30 AM'
			},
			{
				id: 'm74',
				role: 'assistant',
				content:
					"mineral SPF preferably. Think zinc oxide or titanium dioxide. It's physical blocking vs chemical which can irritate sensitive skin. Like the difference between explicit typing and implicit any. 🧱",
				timestamp: '08:31 AM'
			},
			{
				id: 'm75',
				role: 'user',
				content: 'Is retinol actually worth it? Everyone talks about it',
				timestamp: '08:35 AM'
			},
			{
				id: 'm76',
				role: 'assistant',
				content:
					"Retinol is the GOLD standard for anti-aging. It cell turnover, reduces wrinkles, clears acne. But start SLOW. Twice a week max. It's powerful stuff.",
				timestamp: '08:36 AM'
			},
			{
				id: 'm77',
				role: 'user',
				content: 'What happens if I use it every day?',
				timestamp: '08:40 AM'
			},
			{
				id: 'm78',
				role: 'assistant',
				content:
					"Your skin will PURGE. Red, peeling, breakouts. It's the retinization period. Think of it like when you refactor a whole system at once—chaos before clarity. 📈",
				timestamp: '08:41 AM'
			},
			{
				id: 'm79',
				role: 'user',
				content: 'Moisturizer though? Do I need that even if I have oily skin?',
				timestamp: '08:45 AM'
			},
			{
				id: 'm80',
				role: 'assistant',
				content:
					'ESPECIALLY if you have oily skin! When your skin is dry it overcompensates with MORE oil. Moisture tells your skin "we\'re good, no need to panic produce sebum."',
				timestamp: '08:46 AM'
			},
			{
				id: 'm81',
				role: 'user',
				content: 'Retinol is not for the faint of heart.',
				timestamp: '08:50 AM'
			},
			{
				id: 'm82',
				role: 'assistant',
				content:
					"You get it now! Start gentle, listen to your skin, and SPF EVERY SINGLE DAY even when it's cloudy. That's the real secret. ☁️",
				timestamp: '08:51 AM'
			}
		]
	},
	{
		id: '12',
		title: 'Closet full, nothing to wear',
		lastMessage: 'Capsule wardrobe sounds kind of chic actually.',
		messages: [
			{
				id: 'm83',
				role: 'user',
				content: 'I have a full closet but literally wear the same 3 things. Help? 😭',
				timestamp: '09:00 AM'
			},
			{
				id: 'm84',
				role: 'assistant',
				content:
					"Classic capsule wardrobe moment. It's like having 47 npm packages but only using 3. Time to Marie Kondo that closet, bestie! 👗",
				timestamp: '09:01 AM'
			},
			{
				id: 'm85',
				role: 'user',
				content: 'What IS a capsule wardrobe exactly?',
				timestamp: '09:05 AM'
			},
			{
				id: 'm86',
				role: 'assistant',
				content:
					'A limited collection of versatile pieces that all work together. Like a well-organized component library—everything pairs with everything. 🧩',
				timestamp: '09:06 AM'
			},
			{
				id: 'm87',
				role: 'user',
				content: 'How many pieces do I actually need?',
				timestamp: '09:10 AM'
			},
			{
				id: 'm88',
				role: 'assistant',
				content:
					"30-40 pieces is the sweet spot for most people. Including shoes! Think of it like keeping your bundle size small—only what's essential. 🎯",
				timestamp: '09:11 AM'
			},
			{
				id: 'm89',
				role: 'user',
				content: 'Where do I even start decluttering?',
				timestamp: '09:15 AM'
			},
			{
				id: 'm90',
				role: 'assistant',
				content:
					'Pull EVERYTHING out. Yes, the whole closet. Put it on your bed. Then ask yourself: have I worn this in 6 months? Does it fit? Does it bring joy? If no to any—bye bestie! 👋',
				timestamp: '09:16 AM'
			},
			{
				id: 'm91',
				role: 'user',
				content: 'But what if I need it for a specific occasion?',
				timestamp: '09:20 AM'
			},
			{
				id: 'm92',
				role: 'assistant',
				content:
					"One formal dress. One nice blouse. One pair of heels. You don't need 12 formal dresses for 12 years of galas. Rent for special events! 💃",
				timestamp: '09:21 AM'
			},
			{
				id: 'm93',
				role: 'user',
				content: 'What about fast fashion? Those prices are amazing...',
				timestamp: '09:25 AM'
			},
			{
				id: 'm94',
				role: 'assistant',
				content:
					"I KNOW the prices are giving but the quality is giving paper bag. It falls apart after 2 washes. It's like buying the cheapest mechanical keyboard—fun for 2 weeks then you're mad. ⌨️",
				timestamp: '09:26 AM'
			},
			{
				id: 'm95',
				role: 'user',
				content: 'So invest in quality basics?',
				timestamp: '09:30 AM'
			},
			{
				id: 'm96',
				role: 'assistant',
				content:
					'Exactly! A good white tee, well-fitted jeans, neutral blazer. Those are your utility functions. They go with EVERYTHING. 🛍️',
				timestamp: '09:31 AM'
			},
			{
				id: 'm97',
				role: 'user',
				content: 'What about colors? I always default to black...',
				timestamp: '09:35 AM'
			},
			{
				id: 'm98',
				role: 'assistant',
				content:
					"Black is fine but SO boring. Try a capsule in neutrals—cream, camel, olive, navy. Everything mixes and matches. It's like having a consistent design system. 🎨",
				timestamp: '09:36 AM'
			},
			{
				id: 'm99',
				role: 'user',
				content: "OLIVE?? That's not even a real color bestie 😂",
				timestamp: '09:40 AM'
			},
			{
				id: 'm100',
				role: 'assistant',
				content:
					"It IS a real color and it's CHIC. Army green vibes are giving main character energy. Trust me on this one. It goes with everything including black which is illegal but works. 🤯",
				timestamp: '09:41 AM'
			},
			{
				id: 'm101',
				role: 'user',
				content: 'What shoes should I invest in?',
				timestamp: '09:45 AM'
			},
			{
				id: 'm102',
				role: 'assistant',
				content:
					"One white sneaker, one black ankle boot, one nude heel. That's the footwear holy trinity. They go with 90% of outfits. Just like primary colors in design. 👠",
				timestamp: '09:46 AM'
			},
			{
				id: 'm103',
				role: 'user',
				content: 'Accessories though? I feel like I need more...',
				timestamp: '09:50 AM'
			},
			{
				id: 'm104',
				role: 'assistant',
				content:
					'A silk scarf, one statement necklace, simple gold hoops. THATS IT. Accessories are like emoji in texts—sparingly or it gets chaotic. Less is more babe! ✨',
				timestamp: '09:51 AM'
			},
			{
				id: 'm105',
				role: 'user',
				content: 'Capsule wardrobe sounds kind of chic actually.',
				timestamp: '09:55 AM'
			},
			{
				id: 'm106',
				role: 'assistant',
				content:
					'It IS chic! And practical! No more morning outfit panic. Just grab and go. Your future self will thank you. No decision fatigue = better vibes. 🌅',
				timestamp: '09:56 AM'
			}
		]
	},
	{
		id: '13',
		title: 'Gloss vs the world',
		lastMessage: 'Gloss is having a full comeback arc.',
		messages: [
			{
				id: 'm107',
				role: 'user',
				content: 'Lip gloss is back?? I thought we were over that sticky era.',
				timestamp: '12:00 PM'
			},
			{
				id: 'm108',
				role: 'assistant',
				content:
					"BESTIE gloss never left. It was just on a villain arc. Now it's back and better than ever. The formulas are SO much better now. Less sticky, more juicy! 💋",
				timestamp: '12:01 PM'
			},
			{
				id: 'm109',
				role: 'user',
				content: "But isn't lip gloss just... sticky and old lady?",
				timestamp: '12:05 PM'
			},
			{
				id: 'm110',
				role: 'assistant',
				content:
					"That's the OLD gloss. Think Fenty Gloss Bomb. That stuff is ENDORSED. It's plush, not sticky, and gives the most beautiful wet-look shine. Main character energy only. ✨",
				timestamp: '12:06 PM'
			},
			{
				id: 'm111',
				role: 'user',
				content: 'What about lipstick? I feel like that\'s more "put together"',
				timestamp: '12:10 PM'
			},
			{
				id: 'm112',
				role: 'assistant',
				content:
					"Lipstick is giving structured power meeting energy. It's the TypeScript of lips—defined, precise, sometimes intimidating. But gloss? Gloss is playful, approachable, like vanilla JavaScript. 🖤",
				timestamp: '12:11 PM'
			},
			{
				id: 'm113',
				role: 'user',
				content: 'What about lip oils? Are those different?',
				timestamp: '12:15 PM'
			},
			{
				id: 'm114',
				role: 'assistant',
				content:
					"Lip oils are gloss's softer younger sister! They're sheer, super glossy, and super hydrating. It's like the difference between a highlighter and a blinding highlight. Both glow, different vibes. 💫",
				timestamp: '12:16 PM'
			},
			{
				id: 'm115',
				role: 'user',
				content: 'And lip stains? My friend swears by them',
				timestamp: '12:20 PM'
			},
			{
				id: 'm116',
				role: 'assistant',
				content:
					'Lip stains are the minimalist of the lip world. They\'re like that "my lips but better" look. Long-lasting, natural, low maintenance. It\'s the state management approach to lips. Just set it and forget it. 👄',
				timestamp: '12:21 PM'
			},
			{
				id: 'm117',
				role: 'user',
				content: "But don't stains dry out your lips?",
				timestamp: '12:25 PM'
			},
			{
				id: 'm118',
				role: 'assistant',
				content:
					"SOME do. The alcohol-based ones can be drying. But the newer gel or oil-based stains? Chef's kiss. Hydrating AND long-wear. It's like finding the perfect hydration-to-matte balance in a foundation. �基底",
				timestamp: '12:26 PM'
			},
			{
				id: 'm119',
				role: 'user',
				content: "What's the deal with lip liners then?",
				timestamp: '12:30 PM'
			},
			{
				id: 'm120',
				role: 'assistant',
				content:
					"Lip liner is the outline of your lip look. It's like component boundaries—keeps everything in place. Overline if you want more volume, natural line if you're going for subtle. 🎨",
				timestamp: '12:31 PM'
			},
			{
				id: 'm121',
				role: 'user',
				content: 'Do I need to match my liner to my gloss or lipstick?',
				timestamp: '12:35 PM'
			},
			{
				id: 'm122',
				role: 'assistant',
				content:
					"Rule of thumb: liner slightly darker than your lip product. It defines without looking obvious. Unless you're going for that Y2K ombre lip vibe which is also having a moment. 🌪️",
				timestamp: '12:36 PM'
			},
			{
				id: 'm123',
				role: 'user',
				content: 'Wait ombre lips are back too??',
				timestamp: '12:40 PM'
			},
			{
				id: 'm124',
				role: 'assistant',
				content:
					"YES. Everything from the early 2000s is back. Ombre, frosty lips, even lip glaciers. It's giving nostalgic callback energy. Like when frameworks circle back to being popular. 🔄",
				timestamp: '12:41 PM'
			},
			{
				id: 'm125',
				role: 'user',
				content: "What's a lip glacier? That sounds cold.",
				timestamp: '12:45 PM'
			},
			{
				id: 'm126',
				role: 'assistant',
				content:
					'Gloss with icy shimmer or blue tint. Very "ice princess sysadmin". Very cool-toned, very extra, very 2024. Some people are INTO it. I am not one of those people. ❄️',
				timestamp: '12:46 PM'
			},
			{
				id: 'm127',
				role: 'user',
				content: "So what's your daily lip look?",
				timestamp: '12:50 PM'
			},
			{
				id: 'm128',
				role: 'assistant',
				content:
					"TBH? Lip oil or gloss on days I want to look alive. If I'm going to a meeting I'll do a bullet lipstick—classic red or dusty nude. It's like choosing dark mode or light mode. Both valid, depends on the vibe. 🌗",
				timestamp: '12:51 PM'
			},
			{
				id: 'm129',
				role: 'user',
				content: 'What about lip plumping glosses? Do those actually work?',
				timestamp: '12:55 PM'
			},
			{
				id: 'm130',
				role: 'assistant',
				content:
					"The tingle effect? It's temporary swelling from irritants like menthol or capsicum. It works short-term but can be drying. Like a quick caffeine fix vs proper rest. Know what you're signing up for! ☕",
				timestamp: '12:56 PM'
			},
			{
				id: 'm131',
				role: 'user',
				content: 'Gloss is having a full comeback arc.',
				timestamp: '01:00 PM'
			},
			{
				id: 'm132',
				role: 'assistant',
				content:
					"It really is! From glossy skin to glossy lips, it's the texture of the moment. Embrace the shine bestie. Your lips will thank you for the moisture and the glow. ✨💋✨",
				timestamp: '01:01 PM'
			}
		]
	}
];
