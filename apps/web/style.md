# L-Platform: Design System Based on Tailwind v4 & Shadcn/UI

## 1\. Core Philosophy

### Tech Stack

- **Framework:** React (Vite)
- **Styling:** Tailwind v4 (CSS-based)
- **Components:** Shadcn/UI (based on Radix UI)
- **State:** CVA (Class Variance Authority)

### Design Philosophy

This system is built for professional, data-driven esports event platforms. We adopt a **"Dark Mode First"** design strategy to ensure optimal data readability, interaction clarity, and visual hierarchy under the LOL brand's visual tone.

All configurations (including themes, variables, plugins) are defined in the main CSS file.

---

## 3\. Typography System

The typography system remains unchanged, continuing to use Tailwind's utility classes.

| Purpose              | Recommended Classes (Tailwind)                          | Effect          |
| :------------------- | :------------------------------------------------------ | :-------------- |
| Large Display Title  | `text-4xl font-extrabold tracking-tight lg:text-5xl`    | Main Title      |
| Medium Title         | `text-2xl font-semibold tracking-tight`                 | Card Title      |
| Small Title          | `text-xl font-semibold tracking-tight`                  | Sub-section     |
| Label/Category       | `text-sm font-semibold uppercase text-muted-foreground` | Headers, Tags   |
| Primary Body         | `text-base text-foreground`                             | Default Text    |
| Secondary Body       | `text-sm text-muted-foreground`                         | Supplementary   |
| Small Label          | `text-xs text-muted-foreground`                         | Timestamps      |
| Monospace Numbers    | `font-mono text-sm`                                     | KDA, Points     |

**Text Handling:**

- **Truncate:** `truncate`
- **No Wrap:** `whitespace-nowrap`
- **Uppercase:** `uppercase`

---

## 4\. Layout & Spacing

The system uses Tailwind's standard spacing scale (1 = 0.25rem = 4px).

### Spacing Conversion

| Original   | Tailwind Class | Notes         |
| :--------- | :------------- | :------------ |
| `gap_8`    | `gap-2`        | 0.5rem        |
| `gap_16`   | `gap-4`        | 1rem (default)|
| `gap_24`   | `gap-6`        | 1.5rem        |
| `gap_32`   | `gap-8`        | 2rem          |

### Common Padding Patterns

| Pattern      | Tailwind Class      | Usage           |
| :----------- | :------------------ | :-------------- |
| Card/Container| `p-4` or `p-6`     | 16px or 24px    |
| Table Cell   | `py-3 px-4`         | (Shadcn default)|
| Page Container| `p-4 sm:p-6 lg:p-8`| Responsive padding|

### Flexbox & Grid

Use Tailwind classes directly, no `d_flex` prefix needed.

- `d_flex flex-d_column ai_center jc_space-between`
  → `flex flex-col items-center justify-between`
- `flex_1_1_auto` or `flex-g_1`
  → `flex-1`

---

## 5\. Component Guide

Continue using Shadcn/UI components as the foundation.

### Core Components

- **Button (`<Button />`)**
  - `variant="default"`: Primary action (uses `primary` color).
  - `variant="secondary"`: Secondary action (uses `secondary` color).
  - `variant="destructive"`: Red button.
  - `variant="ghost"`: Ghost button (for icon buttons).
- **Card (`<Card />`)**
  - Contains `<CardHeader>`, `<CardTitle>`, `<CardContent>`, etc.
  - Auto-applies `bg-card`, `border`, `rounded-lg` (medium radius).
- **Table (`<Table />`)**
  - `<TableHeader>`: Auto-applies `text-muted-foreground` and `uppercase`.
  - `<TableRow>`: Auto-applies `hover:bg-accent`.

### Custom Component Examples

#### Example 1: Global Power Ranking Table (GPR Table)

```tsx
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "@/components/ui/table";

function GPRTable({ teams }) {
  return (
    <Table>
      <TableHeader>
        <TableRow>
          <TableHead className="w-20">Rank</TableHead>
          <TableHead>Team</TableHead>
          <TableHead className="text-right">Points</TableHead>
        </TableRow>
      </TableHeader>
      <TableBody>
        {teams.map((team) => (
          <TableRow key={team.id} className="hover:bg-accent/80">
            <TableCell className="font-medium">{team.rank}</TableCell>
            <TableCell>
              <div className="flex items-center gap-3">
                <img src={team.logo} alt={team.name} className="h-8 w-8" />
                <span className="font-semibold">{team.name}</span>
              </div>
            </TableCell>
            <TableCell className="text-right font-mono">{team.points}</TableCell>
          </TableRow>
        ))}
      </TableBody>
    </Table>
  );
}
```

#### Example 2: Event Timeline

```tsx
function EventTimelineItem({ children, isLast = false }) {
  return (
    <li className="relative flex gap-4 pb-8">
      {/* Vertical line connector */}
      {!isLast && <div className="bg-border absolute top-4 left-4 -ml-px h-full w-0.5" />}

      {/* Dot */}
      <div className="bg-secondary relative z-10 flex h-8 w-8 items-center justify-center rounded-full">
        <div className="bg-foreground h-2 w-2 rounded-full" />
      </div>

      {/* Content */}
      <div className="flex-1">
        <div className="bg-card hover:bg-accent rounded-md border p-4 transition-colors">{children}</div>
      </div>
    </li>
  );
}
```

---

## 6\. States & Interactions

Interaction states are driven by Tailwind modifiers (`hover:`, `focus-visible:`) and Radix UI's `data-*` attributes.

- **Hover:**
  - `hover:bg-accent` (for cards, rows, menu items)
  - `hover:bg-secondary/80` (for secondary buttons)
- **Focus:**
  - `focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2`
- **Expanded:**
  - `data-[state=open]:bg-accent`

---

## 7\. Responsive Design

We use Tailwind's standard breakpoints (`sm`, `md`, `lg`, `xl`). The `2xl` breakpoint is dedicated to `container` via `@theme` in `style.css`.

**Reverse Breakpoints (max-width):**

- `lg:d_none` → `lg:hidden` (hide on large screens)
- `smDown:p_0` → `max-sm:p-0` (apply on small screens and below)

**Example:**

```html
<div class="mx-auto max-w-7xl p-0 sm:p-6 lg:p-8">
  <div class="flex flex-col gap-4 lg:flex-row">
    <main class="flex-1">...</main>
    <aside class="hidden w-64 lg:block">...</aside>
  </div>
</div>
```
