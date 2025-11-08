# L-Platform: 基于 Tailwind v4 & Shadcn/UI 的设计系统

## 1\. 核心理念

### 技术栈

- **框架:** React (Vite)
- **样式:** Tailwind v4 (CSS-based)
- **组件:** Shadcn/UI (基于 Radix UI)
- **状态:** CVA (Class Variance Authority)

### 设计哲学

本系统专为专业的、数据驱动的电竞赛事平台打造。我们采用 **“暗色优先” (Dark Mode First)** 的设计策略，确保在 LOL 品牌的视觉基调下，数据可读性、交互清晰性和视觉层级达到最佳。

所有配置（包括主题、变量、插件）均在主 CSS 文件中定义。

---

## 3\. 排版系统 (Typography)

排版系统不变，继续使用 Tailwind 的功能类。

| 用途         | 推荐类名 (Tailwind)                                     | 效果       |
| :----------- | :------------------------------------------------------ | :--------- |
| 大型显示标题 | `text-4xl font-extrabold tracking-tight lg:text-5xl`    | 主标题     |
| 中型标题     | `text-2xl font-semibold tracking-tight`                 | 卡片标题   |
| 小型标题     | `text-xl font-semibold tracking-tight`                  | 子区域标题 |
| 标签/分类    | `text-sm font-semibold uppercase text-muted-foreground` | 表头、分类 |
| 主正文       | `text-base text-foreground`                             | 默认文本   |
| 次要正文     | `text-sm text-muted-foreground`                         | 辅助说明   |
| 小标签       | `text-xs text-muted-foreground`                         | 时间戳     |
| 等宽数值     | `font-mono text-sm`                                     | KDA、积分  |

**文本处理:**

- **截断:** `truncate`
- **不换行:** `whitespace-nowrap`
- **大写:** `uppercase`

---

## 4\. 布局与间距 (Layout & Spacing)

系统采用 Tailwind 的标准间距尺度 (1 = 0.25rem = 4px)。

### 间距转换

| 原间距   | Tailwind 类 | 备注        |
| :------- | :---------- | :---------- |
| `gap_8`  | `gap-2`     | 0.5rem      |
| `gap_16` | `gap-4`     | 1rem (默认) |
| `gap_24` | `gap-6`     | 1.5rem      |
| `gap_32` | `gap-8`     | 2rem        |

### 常见内边距模式

| 模式       | Tailwind 类         | 用途          |
| :--------- | :------------------ | :------------ |
| 卡片/容器  | `p-4` 或 `p-6`      | 16px 或 24px  |
| 表格单元格 | `py-3 px-4`         | (Shadcn 默认) |
| 页面容器   | `p-4 sm:p-6 lg:p-8` | 响应式内边距  |

### Flexbox & Grid

直接使用 Tailwind 类，无需 `d_flex` 等前缀。

- `d_flex flex-d_column ai_center jc_space-between`
  → `flex flex-col items-center justify-between`
- `flex_1_1_auto` 或 `flex-g_1`
  → `flex-1`

---

## 5\. 组件指南 (Component Guide)

继续使用 Shadcn/UI 组件作为基础。

### 核心组件

- **按钮 (`<Button />`)**
  - `variant="default"`: 主要操作 (使用 `primary` 色)。
  - `variant="secondary"`: 次要操作 (使用 `secondary` 色)。
  - `variant="destructive"`: 红色按钮。
  - `variant="ghost"`: 幽灵按钮 (用于图标按钮)。
- **卡片 (`<Card />`)**
  - 包含 `<CardHeader>`, `<CardTitle>`, `<CardContent>` 等。
  - 自动应用 `bg-card`, `border`, `rounded-lg` (中圆角)。
- **表格 (`<Table />`)**
  - `<TableHeader>`: 自动应用 `text-muted-foreground` 和 `uppercase`。
  - `<TableRow>`: 自动应用 `hover:bg-accent`。

### 定制组件示例

#### 示例 1: 全球排名表格 (GPR Table)

```tsx
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "@/components/ui/table";

function GPRTable({ teams }) {
  return (
    <Table>
      <TableHeader>
        <TableRow>
          <TableHead className="w-[80px]">Rank</TableHead>
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

#### 示例 2: 赛事时间线 (Event Timeline)

```tsx
function EventTimelineItem({ children, isLast = false }) {
  return (
    <li className="relative flex gap-4 pb-8">
      {/* 竖线连接器 */}
      {!isLast && <div className="bg-border absolute top-4 left-4 -ml-px h-full w-0.5" />}

      {/* 圆点 */}
      <div className="bg-secondary relative z-10 flex h-8 w-8 items-center justify-center rounded-full">
        <div className="bg-foreground h-2 w-2 rounded-full" />
      </div>

      {/* 内容 */}
      <div className="flex-1">
        <div className="bg-card hover:bg-accent rounded-md border p-4 transition-colors">{children}</div>
      </div>
    </li>
  );
}
```

---

## 6\. 状态与交互

交互状态由 Tailwind 的修饰符 (`hover:`, `focus-visible:`) 和 Radix UI 的 `data-*` 属性驱动。

- **悬停 (Hover):**
  - `hover:bg-accent` (用于卡片、行、菜单项)
  - `hover:bg-secondary/80` (用于次要按钮)
- **焦点 (Focus):**
  - `focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2`
- **展开 (Expanded):**
  - `data-[state=open]:bg-accent`

---

## 7\. 响应式设计

我们使用 Tailwind 的标准断点 (`sm`, `md`, `lg`, `xl`)。`2xl` 断点通过 `@theme` 在 `style.css` 中被 `container` 专用。

**反向断点 (max-width):**

- `lg:d_none` → `lg:hidden` (在大屏隐藏)
- `smDown:p_0` → `max-sm:p-0` (在小屏及以下应用)

**示例:**

```html
<div class="mx-auto max-w-7xl p-0 sm:p-6 lg:p-8">
  <div class="flex flex-col gap-4 lg:flex-row">
    <main class="flex-1">...</main>
    <aside class="hidden w-64 lg:block">...</aside>
  </div>
</div>
```
