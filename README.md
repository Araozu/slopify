# Slopify

Yet another AI chat UI, powered by SvelteKit 5.

## Tech Stack

- **Framework**: [SvelteKit](https://kit.svelte.dev/) with Svelte 5 (runes)
- **Styling**: [Tailwind CSS](https://tailwindcss.com/) v4 + [shadcn-svelte](https://shadcn-svelte.com/)
- **Icons**: [Phosphor Icons](https://phosphoricons.com/)
- **Tooling**: [Vite+](https://vite.plus/) (vp dev, vp build, vp test)
- **Database**: [Drizzle ORM](https://orm.drizzle.team/) + PostgreSQL
- **Language**: TypeScript

## Features

- Three-column chat layout (sidebar, main chat, message log)
- Dark/light mode with system preference detection
- Glassmorphism design with backdrop blur
- Avatar components with role-based styling
- Scroll area with smooth navigation

## Getting Started

```sh
# Install dependencies
vp install

# Start development server
vp dev

# Build for production
vp build

# Preview production build
vp preview
```

## Database

```sh
# Push schema to database
pnpm db:push

# Generate migrations
pnpm db:generate

# Apply migrations
pnpm db:migrate

# Open Drizzle Studio
pnpm db:studio
```

## Environment

Copy `.env.example` to `.env` and configure your database URL:

```env
DATABASE_URL="postgres://user:password@host:port/db-name"
```
