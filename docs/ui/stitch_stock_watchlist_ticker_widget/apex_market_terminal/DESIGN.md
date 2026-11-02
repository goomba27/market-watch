---
name: Apex Market Terminal
colors:
  surface: '#0f131c'
  surface-dim: '#0f131c'
  surface-bright: '#353942'
  surface-container-lowest: '#0a0e16'
  surface-container-low: '#181c24'
  surface-container: '#1c2028'
  surface-container-high: '#262a33'
  surface-container-highest: '#31353e'
  on-surface: '#dfe2ee'
  on-surface-variant: '#bdc8d1'
  inverse-surface: '#dfe2ee'
  inverse-on-surface: '#2c3039'
  outline: '#87929a'
  outline-variant: '#3e484f'
  surface-tint: '#7bd0ff'
  primary: '#8ed5ff'
  on-primary: '#00354a'
  primary-container: '#38bdf8'
  on-primary-container: '#004965'
  inverse-primary: '#00668a'
  secondary: '#4edea3'
  on-secondary: '#003824'
  secondary-container: '#00a572'
  on-secondary-container: '#00311f'
  tertiary: '#ffbcb7'
  on-tertiary: '#68000a'
  tertiary-container: '#ff938c'
  on-tertiary-container: '#8d0012'
  error: '#ffb4ab'
  on-error: '#690005'
  error-container: '#93000a'
  on-error-container: '#ffdad6'
  primary-fixed: '#c4e7ff'
  primary-fixed-dim: '#7bd0ff'
  on-primary-fixed: '#001e2c'
  on-primary-fixed-variant: '#004c69'
  secondary-fixed: '#6ffbbe'
  secondary-fixed-dim: '#4edea3'
  on-secondary-fixed: '#002113'
  on-secondary-fixed-variant: '#005236'
  tertiary-fixed: '#ffdad7'
  tertiary-fixed-dim: '#ffb3ad'
  on-tertiary-fixed: '#410004'
  on-tertiary-fixed-variant: '#930013'
  background: '#0f131c'
  on-background: '#dfe2ee'
  surface-variant: '#31353e'
typography:
  headline-xl:
    fontFamily: Plus Jakarta Sans
    fontSize: 32px
    fontWeight: '700'
    lineHeight: 40px
    letterSpacing: -0.02em
  headline-lg:
    fontFamily: Plus Jakarta Sans
    fontSize: 24px
    fontWeight: '600'
    lineHeight: 32px
    letterSpacing: -0.015em
  headline-md:
    fontFamily: Plus Jakarta Sans
    fontSize: 20px
    fontWeight: '600'
    lineHeight: 28px
    letterSpacing: -0.01em
  headline-sm:
    fontFamily: Plus Jakarta Sans
    fontSize: 16px
    fontWeight: '600'
    lineHeight: 24px
    letterSpacing: -0.005em
  body-lg:
    fontFamily: Plus Jakarta Sans
    fontSize: 16px
    fontWeight: '400'
    lineHeight: 24px
    letterSpacing: 0em
  body-md:
    fontFamily: Plus Jakarta Sans
    fontSize: 14px
    fontWeight: '400'
    lineHeight: 20px
    letterSpacing: 0em
  body-sm:
    fontFamily: Plus Jakarta Sans
    fontSize: 12px
    fontWeight: '400'
    lineHeight: 16px
    letterSpacing: 0.01em
  metric-display:
    fontFamily: JetBrains Mono
    fontSize: 24px
    fontWeight: '600'
    lineHeight: 28px
    letterSpacing: -0.03em
  metric-lg:
    fontFamily: JetBrains Mono
    fontSize: 18px
    fontWeight: '500'
    lineHeight: 22px
    letterSpacing: -0.02em
  metric-md:
    fontFamily: JetBrains Mono
    fontSize: 14px
    fontWeight: '500'
    lineHeight: 18px
    letterSpacing: -0.01em
  metric-sm:
    fontFamily: JetBrains Mono
    fontSize: 12px
    fontWeight: '500'
    lineHeight: 16px
    letterSpacing: 0em
  label-caps:
    fontFamily: JetBrains Mono
    fontSize: 10px
    fontWeight: '600'
    lineHeight: 14px
    letterSpacing: 0.08em
rounded:
  sm: 0.125rem
  DEFAULT: 0.25rem
  md: 0.375rem
  lg: 0.5rem
  xl: 0.75rem
  full: 9999px
spacing:
  gutter: 0.75rem
  gutter-desktop: 1rem
  margin: 1rem
  margin-desktop: 1.5rem
  space-xs: 0.25rem
  space-sm: 0.5rem
  space-md: 0.75rem
  space-lg: 1rem
  space-xl: 1.5rem
---

## Brand & Style

This design system delivers a high-density, institutional-grade market monitoring experience built for active traders, modern retail investors, and quantitative analysts. The visual language balances precision engineering with immediate visual telemetry, prioritizing zero-latency comprehension and effortless scanning under high-velocity market conditions.

The aesthetic fuses **Modern Technical Dark Mode** with **Subtle Neon Luminescence**. Deep charcoal and obsidian layers eliminate visual fatigue during extended sessions, while disciplined high-contrast accents (electric cyan, precision emerald, and calibrated crimson) direct attention directly to directional momentum, volatility, and order states. The overall posture avoids decorative noise in favor of razor-sharp data fidelity, low-radius tactile containers, and compact analytical instrumentation.

## Colors

The palette is engineered around dark-room ergonomic safety and rapid semantic identification.

### Background & Surface Hierarchy
- **Canvas Base (`#0B0F17`)**: The deepest obsidian foundation, reserved for main canvas backdrops and recessed telemetry trays.
- **Surface Elevation 1 (`#131A2A`)**: Base card backgrounds, modular dashboard tiles, and tab selector track strips.
- **Surface Elevation 2 (`#1E293B`)**: Interactive hovered elements, secondary popovers, input fields, and pill wrappers.
- **Surface Elevation 3 (`#283548`)**: Active states, dropdown panels, and elevated modal sheets.

### Semantic Performance & Accents
- **Primary Accent (`#38BDF8`)**: Electric cyan used for focus indicators, selection tabs, crosshairs, active sparkline needles, and high-priority terminal alerts.
- **Bullish / Gain (`#10B981`)**: Saturated emerald green for positive delta values, buy volumes, upward order ticks, and positive trend lines. Paired with a 10% opacity wash (`rgba(16, 185, 129, 0.12)`) for status pills.
- **Bearish / Loss (`#EF4444`)**: Crisp crimson red for negative delta values, sell volumes, downward price ticks, and critical threshold breaches. Paired with a 10% opacity wash (`rgba(239, 68, 68, 0.12)`).
- **Subtle Borders & Separators (`#1E293B` to `#334155`)**: Semi-transparent, structural boundary rules that keep metrics segmented without introducing contrast clashing.
- **Text & Numeric Hierarchy**:
  - Primary metric text: `#F8FAFC`
  - Secondary metadata & column headers: `#94A3B8`
  - Disabled or inactive tickers: `#475569`

## Typography

The typographic hierarchy implements a bifurcated dual-engine strategy: **Plus Jakarta Sans** governs macro interface wayfinding, titles, structural labels, and analytical narratives; **JetBrains Mono** governs all numeric figures, stock tickers, percentage badges, time stamps, and order book readouts.

### Tabular Alignment & Readability
- JetBrains Mono provides tabular proportional numbers by default, preventing visual jitter and layout shifting when asset prices tick in real-time.
- Micro-labels (`label-caps`) use uppercase tracking to enhance distinctness within dense table headings and asset classifications.
- All monetary signs (`$`, `€`, `¥`) and directional indicators (`+`, `-`, `▲`, `▼`) share exact vertical baselines and width metrics with numeric digits.

## Layout & Spacing

The layout operates on a compact 4px baseline sub-grid tailored for high-density financial consoles and responsive widget embedding.

### Density Philosophy
- Standard padding tiers are constrained (`space-sm` for metric inner cells, `space-md` for standard card bodies) to maximize viewport real estate while maintaining sufficient breathing room for rapid tap targets.
- Desktop layouts utilize a modular 12-column fluid grid system with `gutter-desktop` (16px), enabling 3-column, 4-column, or multi-split terminal layouts.
- Tablet interfaces collapse secondary analytical panels into swipeable panels or drawer overlays, keeping price tickers anchored.
- Mobile layouts step down to a single-column stacked view with tight margins (`1rem`) and edge-to-edge scroll horizontal rail tabs.

## Elevation & Depth

Visual depth is achieved primarily through dark **tonal layering** and **subtle luminous glows**, eschewing traditional diffuse drop shadows that create muddiness on OLED and dark displays.

### Level Calibration
- **Tier 0 (Backdrop / Canvas)**: `#0B0F17`. Flat base layer with no shadow.
- **Tier 1 (Cards, Modules, Trackers)**: `#131A2A` bounded by a 1px crisp outline: `rgba(255, 255, 255, 0.06)`. Subtle ambient shadow: `0 4px 12px rgba(0, 0, 0, 0.4)`.
- **Tier 2 (Floating Action Bars & Dropdown Flyouts)**: `#1E293B` bounded by `rgba(56, 189, 248, 0.15)`. Ambient shadow: `0 8px 24px rgba(0, 0, 0, 0.6)`.
- **Tier 3 (Active Sheets & Modals)**: `#1E293B` elevated above an obsidian scrim (`rgba(11, 15, 23, 0.8)` with `backdrop-filter: blur(12px)`). Border: `1px solid rgba(255, 255, 255, 0.12)`. Shadow: `0 16px 40px rgba(0, 0, 0, 0.8)`.

### Luminous Indicators
- Gain elements feature a controlled micro-glow under active states: `box-shadow: 0 0 12px rgba(16, 185, 129, 0.3)`.
- Loss elements utilize matching crimson illumination: `box-shadow: 0 0 12px rgba(239, 68, 68, 0.3)`.
- Focused terminal controls feature electric cyan rim-lighting: `0 0 0 1px #38BDF8, 0 0 8px rgba(56, 189, 248, 0.35)`.

## Shapes

The design system adopts a **Soft (`1`)** shape geometry, prioritizing crisp, structured edges over hyper-rounded silhouettes to preserve data density and evoke professional hardware terminals.

- Micro indicators, delta tags, and sparkline containers utilize `0.25rem` (4px).
- Cards, list rows, tabs, and input wrappers leverage `rounded-lg` (`0.5rem` / 8px).
- Elevated sheets and modal containers scale to `rounded-xl` (`0.75rem` / 12px).
- Circular profiles and icon action triggers are strictly restricted to true geometry (`rounded-full`), while data pills maintain softened rectangular framing to preserve rectangular monospace typographic rhythms.

## Components

### Buttons & Interactive Triggers
- **Primary Action (Execute / Add Symbol)**: Background `#38BDF8`, text `#0B0F17` (bold `Plus Jakarta Sans`), hover `#7DD3FC`. Subtle electric ambient ring on focus.
- **Secondary Action (Manage / Cancel)**: Background `#1E293B`, text `#F8FAFC`, border `1px solid rgba(255, 255, 255, 0.08)`. Hover background `#283548`.
- **Destructive Action (Remove Watchlist / Delete Position)**: Background `rgba(239, 68, 68, 0.12)`, text `#EF4444`, border `1px solid rgba(239, 68, 68, 0.25)`. Hover background `rgba(239, 68, 68, 0.2)`.

### Metric Chips & Delta Badges
- Gain Badge: Emerald background (`rgba(16, 185, 129, 0.12)`), text `#10B981`, font `JetBrains Mono` (`metric-sm`), 4px corner radius, padding `2px 6px`.
- Loss Badge: Crimson background (`rgba(239, 68, 68, 0.12)`), text `#EF4444`, font `JetBrains Mono` (`metric-sm`), 4px corner radius, padding `2px 6px`.
- Neutral Badge: Charcoal background (`rgba(148, 163, 184, 0.1)`), text `#94A3B8`.

### Watchlist Tabs & Segmented Switchers
- Housed within an Elevation 1 track (`#131A2A`) with `space-xs` inset padding.
- Active tab displays a solid background `#1E293B`, electric cyan underline indicator (`#38BDF8`, 2px bottom border or dot), and primary white text. Inactive tabs display `#94A3B8` without borders.
- Includes a terminal "+" action button at the trailing edge of the horizontal rail for quick watchlist creation.

### Market List Rows & Cards
- List item heights locked to a dense 56px (mobile) or 48px (compact desktop table).
- Left-aligned ticker badge, asset name in muted tone underneath. Right-aligned current price in `metric-md` with percentage change badge nested immediately below or alongside.
- Subtle row hover effect: `background: rgba(255, 255, 255, 0.02)`. Active selection highlights with a 2px left border strip in `#38BDF8`.

### Micro Sparklines
- Sized at 64px width × 24px height within list rows, expanding to full container width inside modular summary cards.
- Non-smoothed vector polyline with a 1.5px stroke width. Bullish sparklines use `#10B981` with an SVG linear gradient fill fading to `rgba(16, 185, 129, 0.0)`. Bearish sparklines use `#EF4444` fading to `rgba(239, 68, 68, 0.0)`.
- Interactive tooltip renders an ultra-thin vertical guideline (`#38BDF8`, 1px dashed) with a floating mono popover displaying time and tick price.

### Inputs & Search
- Low-profile inputs with background `#0B0F17`, border `1px solid #1E293B`, and placeholder `#475569`.
- Integrated search icon (`#94A3B8`) on leading edge, with interactive keyboard shortcut indicator (e.g., `⌘K`) rendered in JetBrains Mono on the trailing edge.

### Bottom Sheet & Modal Management Controls
- Mobile watchlist management renders as an interactive bottom sheet sliding up from base, equipped with an ergonomic drag pill (`#334155`, 36px × 4px) at the top.
- Includes quick-reorder drag handles on list rows, bulk-delete checkboxes (`accent-color: #38BDF8`), and real-time symbol lookup with auto-complete chips.