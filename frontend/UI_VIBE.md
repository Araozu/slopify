# Slopify UI Design Philosophy

A quick reference for the visual language to keep all pages consistent.

---

## Spacing & Sizing

- UI chrome (headers, toolbars) is **compact** — `h-8` or less. Content areas breathe, chrome does not.
- Padding inside containers: `p-4` standard, `p-2`/`p-3` for tight controls, `p-6` for open content areas.
- Content is **max-w-3xl** and centered — never full-bleed on large screens.

## Border Radius

Rounded, but not bubbly. Use the scale with intent:

- `rounded-md` — inputs, small controls
- `rounded-xl` — cards, dropdowns, sidebars items
- `rounded-2xl` — message bubbles, callout boxes
- `rounded-3xl` — empty state cards, large feature containers
- `rounded-full` — avatars, icon buttons, dot indicators

## Typography

- Scale stays **small**. `text-sm` is the default readable size, `text-xs` for secondary info.
- Labels, metadata, and section headers: `text-[10px] font-black tracking-widest uppercase` — not `text-xs`, the extra smallness is intentional.
- Timestamps and fine print: `text-[9px]` or `text-[11px]`.
- AI content renders in `font-mono` — everything else uses the sans default.
- Muted text uses opacity modifiers: `text-muted-foreground/40`, `/60`, `/70` rather than a new color.

## Color Usage

- **Never hardcode colors.** Always use CSS variables (`text-primary`, `bg-muted`, etc.).
- Transparency via alpha modifiers (`/10`, `/20`, `/50`) is preferred over separate muted variables.
- Destructive actions: use `text-destructive` and `hover:text-destructive`, not custom reds.
- Active/selected states: `bg-background shadow-sm ring-1 ring-border` — a soft card pop, not a color fill.

## Borders & Depth

- Borders are subtle: `border-border/60` or `ring-1 ring-border` — never heavy.
- Depth comes from **blur + transparency**, not shadows. `backdrop-blur-md/xl` on any floating surface.
- When a shadow is needed: `shadow-sm` or `shadow-[0_2px_10px_-3px_...]` — barely-there elevation.
- Inner inputs/containers use `shadow-inner ring-1 ring-border/50` — recessed, not raised.

## Interactivity

- Hover states: `hover:bg-foreground/10` or `hover:bg-muted/80` — nearly invisible, just a whisper.
- Secondary/destructive actions are **hidden until hover** (`invisible group-hover:visible`) — keep the UI calm at rest.
- Press feedback on primary actions: `active:scale-95`.
- Focus rings: `focus-visible:ring-1 focus-visible:ring-ring` — present but not aggressive.
