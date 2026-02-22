# 🖥 sys-monitor — Product Vision

> A modern, elegant, macOS-first terminal system monitor for developers who value clarity and performance.

---

## Table of Contents

1. [Core Purpose](#core-purpose)
2. [Target Users](#target-users)
3. [Problem Statement](#problem-statement)
4. [Product Philosophy](#product-philosophy)
5. [Information Architecture](#information-architecture)
6. [User Experience Goals](#user-experience-goals)
7. [Visual Identity](#visual-identity)
8. [Color System](#color-system)
9. [Layout & Widget Design](#layout--widget-design)
10. [Interaction Model](#interaction-model)
11. [Technical Architecture](#technical-architecture)
12. [Performance Budget](#performance-budget)
13. [Roadmap](#roadmap)
14. [Positioning](#positioning)

---

## Core Purpose

**sys-monitor** is a terminal-native macOS system monitor that gives developers instant clarity about their machine's performance — without overwhelming them.

It is **not**:
- A clone of `htop`, `btop`, or `glances`
- An old-school hacker aesthetic tool
- A tutorial or toy project

It **is**:
- A minimal, focused performance dashboard
- A premium CLI product with intentional design
- A tool that respects the developer's attention

**One-liner:** *Know your system's health in under 3 seconds.*

---

## Target Users

| Persona | Needs |
|---|---|
| **Terminal-native developer** | Quick glance at system state without leaving the terminal |
| **CS student** | Learning OS concepts through real-time system data |
| **macOS power user** | Lightweight alternative to Activity Monitor |
| **DevOps engineer** | Fast local monitoring without heavy tooling |
| **Minimalist** | Dislikes bloated GUI monitors with 50 tabs |

### Common Traits
- Works in the terminal daily
- Values speed and keyboard-first workflows
- Prefers tools with strong visual hierarchy
- On macOS (Apple Silicon or Intel)

---

## Problem Statement

Existing system monitors fall into one of four failure modes:

| Problem | Examples |
|---|---|
| **Too heavy** | Activity Monitor, iStat Menus — full GUI apps with large footprints |
| **Too cluttered** | `htop`, `glances` — every metric visible at once, no hierarchy |
| **Too technical** | Raw `/proc` output, `vm_stat` — requires expertise to interpret |
| **Poorly designed** | Many TUI tools — flickery, inconsistent spacing, cryptic labels |

### What sys-monitor solves

| Pain Point | Solution |
|---|---|
| Information overload | Curated, hierarchical metrics — only what matters |
| Lack of clarity | Color-coded health indicators with instant readability |
| Slow performance tools | Sub-millisecond render loop, zero-allocation hot path |
| Poor visual hierarchy | Deliberate layout, typography, and spacing |

---

## Product Philosophy

### Design Principles

```
1. CLARITY over complexity
   → Every element has a purpose. No decorative noise.

2. SIGNAL over noise
   → Surface the metrics that matter. Hide the rest.

3. MINIMALISM over density
   → Whitespace is a feature, not wasted space.

4. PERFORMANCE over decoration
   → 60fps rendering. Zero lag. Instant startup.

5. PROGRESSIVE DISCLOSURE
   → Show summaries first. Details on demand.
```

### The "3-Second Rule"

When a user launches sys-monitor, they should understand the system's health state within **3 seconds** by scanning:

1. **CPU** — Is it under load? Which cores are stressed?
2. **Memory** — How close to full? What's the pressure?
3. **Top processes** — Who's consuming resources?
4. **Uptime** — How long has the system been running?

If the user needs to think, read labels, or decode numbers — the design has failed.

---

## Information Architecture

### Primary Dashboard (Default View)

The dashboard is organized into **4 zones**, ordered by priority:

```
┌─────────────────────────────────────────────────────────┐
│                    HEADER BAR                           │
│  sys-monitor  │  hostname  │  uptime  │  load avg      │
├────────────────────────────┬────────────────────────────┤
│                            │                            │
│       CPU PANEL            │      MEMORY PANEL          │
│                            │                            │
│  Overall usage bar         │  RAM usage bar             │
│  Per-core sparklines       │  Swap usage bar            │
│  CPU temperature           │  Memory pressure           │
│                            │  Breakdown (app/wired/     │
│                            │    compressed)             │
│                            │                            │
├────────────────────────────┴────────────────────────────┤
│                                                         │
│                   PROCESS TABLE                         │
│                                                         │
│  PID  │  Name  │  CPU%  │  MEM%  │  State  │  User     │
│  ───  │  ────  │  ────  │  ────  │  ─────  │  ────     │
│  ...  │  ...   │  ...   │  ...   │  ...    │  ...      │
│                                                         │
└─────────────────────────────────────────────────────────┘
```

### Zone Priority

| Zone | Purpose | Glance Time |
|---|---|---|
| **Header** | System identity & uptime context | < 0.5s |
| **CPU Panel** | Is the CPU stressed? Which cores? | < 1s |
| **Memory Panel** | Is memory under pressure? | < 1s |
| **Process Table** | Who is consuming resources? | < 2s |

### Metric Selection Criteria

A metric is included **only if** it answers one of these:
- Is something wrong right now?
- What is causing the problem?
- How long has this been going on?

Metrics that fail this test are **excluded** from v1.

---

## User Experience Goals

### Emotional Qualities

| Quality | Meaning |
|---|---|
| **Calm** | No flashing, no urgency unless warranted |
| **Smooth** | Consistent 60fps, no flickering or tearing |
| **Responsive** | Instant reaction to keyboard input |
| **Clean** | Balanced whitespace, aligned elements |
| **Focused** | One clear purpose per screen region |

### Behavioral Goals

- **Startup in < 200ms** — The tool should be ready before the user's eyes settle
- **No scroll needed** — The default view fits in a standard terminal (80×24 minimum, optimized for 120×40)
- **Keyboard-first** — All navigation via keyboard shortcuts
- **Graceful resize** — Adapts to any terminal size without breaking layout
- **Silent by default** — No sounds, no notifications, no popups

### Anti-Goals

These are things sys-monitor will **never** do:

- ❌ Show every possible metric
- ❌ Require mouse interaction
- ❌ Use aggressive colors by default
- ❌ Flash or blink elements
- ❌ Require configuration to be useful
- ❌ Block on slow system calls

---

## Visual Identity

### Aesthetic DNA

```
Modern macOS terminal aesthetic
├── Clean rounded borders (Ratatui's Rounded block style)
├── Balanced inner padding (1-char horizontal, 0-line vertical)
├── Consistent typography (Unicode box-drawing, no ASCII art)
├── Soft color accents (muted tones, not saturated primaries)
└── Dark-first design (optimized for dark terminal themes)
```

### Typography Hierarchy

| Element | Style |
|---|---|
| Section titles | UPPERCASE, bold, muted color |
| Metric labels | Regular, dim foreground |
| Metric values | Bold, bright foreground |
| Units | Regular, dim, appended to values |
| Status text | Colored by severity |

### Spacing Rules

- **Panel padding:** 1 character horizontal, 0 lines vertical
- **Between panels:** 1 character gap
- **Label-to-value gap:** Right-aligned values within fixed column width
- **Process table rows:** No extra padding, dense but readable

---

## Color System

### Base Palette (Dark Theme)

```
Background:      terminal default (transparent)
Surface:         terminal default
Border:          #555555  (DarkGray — subtle, not invisible)
Text Primary:    #E0E0E0  (bright white, not harsh #FFFFFF)
Text Secondary:  #888888  (dim, for labels and units)
Text Disabled:   #555555  (for inactive elements)
```

### Semantic Colors

```
Healthy:     #5AEEA0  (soft green  — system is fine)
Warning:     #F5C842  (warm amber  — attention needed)
Critical:    #FF6B6B  (soft red    — action required)
Info:        #64B5F6  (calm blue   — neutral information)
Accent:      #BB86FC  (soft purple — highlights and focus)
```

### Dynamic Color Mapping

Colors shift based on metric thresholds — **not binary, but gradient**:

| Metric | 0-50% | 50-75% | 75-90% | 90-100% |
|---|---|---|---|---|
| CPU Usage | Healthy | Info | Warning | Critical |
| Memory Usage | Healthy | Info | Warning | Critical |
| Swap Usage | Healthy | Warning | Critical | Critical |

The transition should feel **organic**, not like a traffic light.

### Gauge Bar Colors

```
CPU gauge fill:    gradient from Healthy → Warning → Critical
Memory gauge fill: gradient from Healthy → Warning → Critical  
Swap gauge fill:   gradient from Info → Warning → Critical
Empty gauge:       #333333 (dark, recedes visually)
```

---

## Layout & Widget Design

### Header Bar

```
╭─ sys-monitor ─────────────────────────────────────────────╮
│  ◉ MacBook Pro  │  macOS 15.3  │  ↑ 3d 14h  │  ⎔ 1.52   │
╰───────────────────────────────────────────────────────────╯
```

- **sys-monitor** — App name, always visible, sets identity
- **Hostname** — Machine name for context (especially useful for future SSH)
- **OS Version** — macOS version
- **Uptime** — Human-readable (e.g., "3d 14h", not "302400 seconds")
- **Load average** — 1-minute load, single number

### CPU Panel

```
╭─ CPU ─────────────────────────╮
│                               │
│  Overall  ████████░░░░  67%   │
│                               │
│  Core 0   ██████░░░░░░  48%   │
│  Core 1   ████████████  99%   │
│  Core 2   ███░░░░░░░░░  22%   │
│  Core 3   █████░░░░░░░  38%   │
│  Core 4   ████████░░░░  65%   │
│  Core 5   ██░░░░░░░░░░  12%   │
│  Core 6   ███████░░░░░  55%   │
│  Core 7   █████████░░░  82%   │
│                               │
│  Temperature:  62°C           │
│  Processes:    312            │
│  Threads:      1847           │
╰───────────────────────────────╯
```

- **Overall bar** — Aggregated CPU usage, largest and topmost
- **Per-core bars** — Individual core loads with color-coded fills
- **Temperature** — CPU package temperature (if available via macOS APIs)
- **Process/Thread count** — Quick process context

### Memory Panel

```
╭─ MEMORY ──────────────────────╮
│                               │
│  RAM     ██████████░░  83%    │
│          13.2 / 16.0 GB       │
│                               │
│  Swap    ██░░░░░░░░░░   8%    │
│          0.5 / 6.0 GB         │
│                               │
│  ┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄  │
│                               │
│  App Memory     8.4 GB        │
│  Wired          3.1 GB        │
│  Compressed     1.7 GB        │
│  Cached         4.2 GB        │
│                               │
│  Pressure:  ● Normal          │
╰───────────────────────────────╯
```

- **RAM bar** — Total physical memory usage with absolute values
- **Swap bar** — Swap usage (warning sign if high)
- **Breakdown** — macOS-specific memory categories
- **Pressure indicator** — macOS memory pressure level (Normal / Warn / Critical)

### Process Table

```
╭─ TOP PROCESSES ────────────────────────────────────────────╮
│                                                            │
│   PID    Name               CPU%     MEM%    State         │
│  ─────  ─────────────────  ──────  ──────  ────────        │
│  1842   Google Chrome       23.4%    8.2%   Running        │
│   491   kernel_task         12.1%    1.4%   Running        │
│  2103   node                 9.8%    3.1%   Running        │
│  1923   Spotlight            7.2%    0.9%   Running        │
│  2287   cargo                5.5%    2.8%   Running        │
│  1102   WindowServer         4.1%    1.2%   Running        │
│   832   Finder               1.3%    0.5%   Running        │
│  2301   Terminal             0.8%    0.3%   Running        │
│                                                            │
│  ↑/↓ Navigate  │  s Sort  │  k Kill  │  / Filter          │
╰────────────────────────────────────────────────────────────╯
```

- **Sorted by CPU% by default** — Most resource-hungry processes first
- **Color-coded CPU%** — High consumers highlighted in warning/critical colors
- **Keyboard hints** — Subtle footer showing available interactions
- **Sortable columns** — Toggle sort by CPU, MEM, PID, Name
- **Filter** — Quick search by process name

---

## Interaction Model

### Keyboard Shortcuts

| Key | Action |
|---|---|
| `q` / `Esc` | Quit |
| `↑` / `k` | Move selection up in process table |
| `↓` / `j` | Move selection down in process table |
| `s` | Cycle sort column (CPU → MEM → PID → Name) |
| `S` | Reverse sort order |
| `/` | Open filter prompt |
| `Enter` | View process details (future) |
| `K` | Kill selected process (with confirmation) |
| `r` | Force refresh |
| `?` | Show help overlay |
| `1` | Toggle per-core CPU view |
| `Tab` | Cycle focus between panels |

### Refresh Behavior

- **Data refresh:** Every **1 second** (configurable)
- **UI render:** Every **250ms** (smooth gauge animations)
- **Process list:** Every **2 seconds** (reduce syscall overhead)

Staggered refresh prevents burst I/O and keeps the display smooth.

---

## Technical Architecture

### Technology Stack

| Layer | Technology | Rationale |
|---|---|---|
| **Language** | Rust | Memory safety, zero-cost abstractions, native performance |
| **TUI Framework** | Ratatui 0.30 | Modern, well-maintained, widget-rich |
| **Terminal Backend** | Crossterm | Cross-platform terminal manipulation (bundled with Ratatui) |
| **System Data** | `sysinfo` crate | Cross-platform system metrics |
| **macOS-specific** | `mach` / `libc` / `IOKit` FFI | Memory pressure, temperature, per-core CPU |
| **Async Runtime** | None (polling) | Simplicity, predictable timing |

### Module Architecture

```
sys-monitor/
├── src/
│   ├── main.rs              # Entry point, terminal setup, event loop
│   ├── app.rs               # Application state and lifecycle
│   ├── event.rs             # Keyboard & tick event handling
│   ├── ui/
│   │   ├── mod.rs           # UI root — composes all panels
│   │   ├── header.rs        # Header bar widget
│   │   ├── cpu.rs           # CPU panel widget
│   │   ├── memory.rs        # Memory panel widget
│   │   ├── processes.rs     # Process table widget
│   │   └── help.rs          # Help overlay widget
│   ├── system/
│   │   ├── mod.rs           # System data aggregation
│   │   ├── cpu.rs           # CPU metrics collector
│   │   ├── memory.rs        # Memory metrics collector
│   │   ├── process.rs       # Process list collector
│   │   └── host.rs          # Hostname, uptime, OS info
│   ├── theme/
│   │   ├── mod.rs           # Theme definitions
│   │   └── colors.rs        # Color palette and severity mapping
│   └── util/
│       ├── mod.rs           # Shared utilities
│       └── format.rs        # Human-readable formatting (bytes, duration, etc.)
├── docs/
│   ├── PRODUCT_VISION.md    # This document
│   └── ARCHITECTURE.md      # Technical deep-dive (future)
├── Cargo.toml
└── README.md
```

### Data Flow

```
┌──────────────┐     ┌──────────────┐     ┌──────────────┐
│   System     │     │   App State  │     │   Terminal    │
│   APIs       │ ──→ │   (struct)   │ ──→ │   Render     │
│              │     │              │     │              │
│  sysinfo     │     │  cpu_data    │     │  Ratatui     │
│  mach/libc   │     │  mem_data    │     │  widgets     │
│  IOKit       │     │  proc_list   │     │  Crossterm   │
└──────────────┘     └──────────────┘     └──────────────┘
       ↑                    ↑
       │                    │
    [1s tick]          [keyboard]
```

### Event Loop

```
loop {
    // 1. Poll for events (keyboard or tick)
    // 2. Handle keyboard input → update app state
    // 3. If tick → refresh system data
    // 4. Render UI from current state
    // 5. Flush to terminal
}
```

Single-threaded, non-blocking, polling-based. No async runtime overhead.

---

## Performance Budget

| Metric | Target | Rationale |
|---|---|---|
| **Startup time** | < 200ms | Faster than the user can blink |
| **Render frame time** | < 5ms | Well within 60fps budget (16ms) |
| **Memory footprint** | < 15 MB RSS | Lighter than any GUI monitor |
| **CPU usage (idle)** | < 1% | Must not show up in its own process list |
| **Binary size** | < 5 MB | Quick install, small footprint |

### Optimization Strategy

- **Differential rendering** — Only redraw changed regions (Ratatui handles this)
- **Staggered data collection** — Spread syscalls across ticks
- **Reuse allocations** — Pre-allocated buffers for process list
- **No unnecessary copies** — Zero-copy string rendering where possible

---

## Roadmap

### v0.1 — Foundation (Current)

- [ ] Terminal setup & teardown (alternate screen, raw mode)
- [ ] Basic event loop (keyboard + tick)
- [ ] App state struct
- [ ] Theme & color system
- [ ] Header bar (hostname, uptime, load)
- [ ] CPU panel (overall + per-core gauges)
- [ ] Memory panel (RAM + swap bars, breakdown)
- [ ] Process table (sortable, scrollable)
- [ ] Keyboard navigation
- [ ] Graceful resize handling
- [ ] Clean error handling & panic recovery

### v0.2 — Polish

- [ ] Smooth gauge animations (interpolated fill)
- [ ] Process filtering (search by name)
- [ ] Process kill with confirmation dialog
- [ ] Help overlay (`?` key)
- [ ] Sort indicator in table headers
- [ ] Sparkline history for CPU (last 60s)
- [ ] Config file support (`~/.config/sys-monitor/config.toml`)

### v0.3 — macOS Deep Integration

- [ ] CPU temperature via IOKit
- [ ] Memory pressure via mach APIs
- [ ] Disk I/O metrics
- [ ] Network throughput
- [ ] Battery status (for MacBooks)

### v1.0 — Release

- [ ] Homebrew formula
- [ ] Man page
- [ ] README with screenshots
- [ ] CI/CD pipeline
- [ ] Integration tests
- [ ] Performance benchmarks

### Future (Post v1.0)

- [ ] Remote server monitoring via SSH
- [ ] Exportable performance logs (CSV/JSON)
- [ ] Multiple machine dashboard
- [ ] AI-based anomaly detection
- [ ] Plugin system for custom widgets
- [ ] GPU monitoring (Apple Silicon)

---

## Positioning

### What sys-monitor IS

> A modern macOS-native terminal monitor for developers who value clarity and performance.

### What it is NOT

| Not This | Why |
|---|---|
| A toy project | Production-quality code, tested, documented |
| An htop clone | Different philosophy, different audience, different UX |
| A cross-platform tool (v1) | macOS-first, Linux/Windows considered later |
| A DevOps suite | Focused on local machine monitoring |
| A configurable dashboard | Opinionated defaults that work out of the box |

### Competitive Landscape

| Tool | Weakness (that sys-monitor solves) |
|---|---|
| `htop` | Dense, no visual hierarchy, no color semantics |
| `btop` | Beautiful but complex, many panels, steep learning curve |
| `glances` | Python-based, slow startup, web UI focus |
| Activity Monitor | Full GUI app, not terminal-native |
| `top` | Raw, no color, no bars, requires expertise |

### sys-monitor's Edge

```
htop    = Everything, everywhere, all at once
btop    = Beautiful but overwhelming
sys-mon = Exactly what you need, beautifully presented
```

---

## Emotional Design Contract

When a user runs `sys-monitor`, they should think:

> *"This feels professional."*
> *"This is well designed."*  
> *"This is better than the default."*
> *"This was built by someone who understands systems."*

When CPU is high → the user should feel **informed**, not panicked.  
When memory is full → the user should see **clarity**, not alarm.  
When everything is fine → the user should feel **confident**, not bored.

The tool earns trust through restraint.

---

## Success Criteria

| Criteria | Measurement |
|---|---|
| **3-second comprehension** | New user understands system state in one glance |
| **Zero configuration** | Works perfectly on first run |
| **Invisible performance** | Never shows up as a resource consumer |
| **Visual consistency** | Every pixel is intentional |
| **Developer trust** | "I leave this running in a split pane all day" |

---

*This document is the north star for sys-monitor v1. Every design decision, every line of code, every widget should trace back to a principle defined here.*
