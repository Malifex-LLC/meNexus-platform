# meNexus Theme Creation Guide

## Overview

meNexus uses two separate theme systems:
- **User Themes** - Controls the main app UI (Control Panel, Dashboard, Messages, etc.)
- **Synapse Themes** - Controls Synapse content (modules, feeds, chat, etc.)

Both systems use **identical color variables**, making it easy to create and adapt themes.

## Quick Start

### Creating a User Theme

1. Copy `user-themes/midnight.css` as your starting point
2. Rename it (e.g., `my-theme.css`)
3. Modify the color values
4. Import it in your main CSS entry point

### Creating a Synapse Theme

1. Copy `synapse-themes/default.css` as your starting point
2. Rename it (e.g., `my-synapse.css`)
3. Modify the color values
4. (Synapse themes are loaded dynamically based on Synapse configuration)

### Converting User Theme → Synapse Theme

1. Copy your user theme file
2. Find & Replace: `--theme-` → `--synapse-`
3. Find & Replace: `:root` → `.synapse-themed`
4. Done!

## Color Variable Reference

### Layout Colors
| Variable | Purpose |
|----------|---------|
| `bg-window` | Main window/app background |
| `bg-panel` | Sidebars, navigation panels |
| `bg-surface` | Cards, dropdowns, elevated surfaces |
| `bg-elevated` | Modals, popovers, tooltips |
| `bg-input` | Input field backgrounds |
| `bg-overlay` | Modal backdrop overlay |

### Border Colors
| Variable | Purpose |
|----------|---------|
| `border` | Standard borders |
| `border-subtle` | Very subtle/light borders |
| `border-strong` | Emphasized borders |

### Text Colors
| Variable | Purpose |
|----------|---------|
| `text-primary` | Main body text |
| `text-secondary` | Secondary/muted text |
| `text-subtle` | Placeholders, hints |
| `text-disabled` | Disabled state text |

### Accent Colors
| Variable | Purpose |
|----------|---------|
| `accent` | Primary brand/accent color |
| `accent-hover` | Accent hover state |
| `accent-muted` | Muted accent (for backgrounds) |
| `accent-text` | Text on accent backgrounds |
| `secondary` | Secondary accent color |
| `secondary-hover` | Secondary hover state |

### Interactive States
| Variable | Purpose |
|----------|---------|
| `hover` | General hover backgrounds |
| `active` | Active/pressed states |
| `focus` | Focus ring color |

### Status Indicators
| Variable | Purpose |
|----------|---------|
| `status-online` | Online indicator (emerald) |
| `status-offline` | Offline indicator (gray) |
| `status-busy` | Busy/DND indicator (red) |
| `status-away` | Away indicator (amber) |
| `status-streaming` | Live/streaming (rose) |

### Semantic Colors
| Variable | Purpose |
|----------|---------|
| `success` / `success-muted` | Success states |
| `warning` / `warning-muted` | Warning states |
| `error` / `error-muted` | Error states |
| `info` / `info-muted` | Info states |

### Role Colors
| Variable | Purpose |
|----------|---------|
| `role-owner` / `role-owner-bg` | Owner role |
| `role-admin` / `role-admin-bg` | Admin role |
| `role-moderator` / `role-moderator-bg` | Moderator role |
| `role-member` / `role-member-bg` | Member role |

### Message Colors
| Variable | Purpose |
|----------|---------|
| `msg-own-bg` / `msg-own-text` | Your messages |
| `msg-other-bg` / `msg-other-text` | Others' messages |
| `msg-system-bg` / `msg-system-text` | System messages |

### Stat/Metric Colors
| Variable | Purpose |
|----------|---------|
| `stat-primary` | Primary stat color |
| `stat-secondary` | Secondary stat color |
| `stat-positive` | Positive changes (green) |
| `stat-negative` | Negative changes (red) |

### Reputation Levels
| Variable | Purpose |
|----------|---------|
| `rep-newcomer` | Score 0-99 |
| `rep-contributor` | Score 100-499 |
| `rep-established` | Score 500-999 |
| `rep-trusted` | Score 1000-2499 |
| `rep-respected` | Score 2500-4999 |
| `rep-distinguished` | Score 5000-9999 |
| `rep-elite` | Score 10000-24999 |
| `rep-legendary` | Score 25000+ |

### Notification Colors
| Variable | Purpose |
|----------|---------|
| `notif-mention` | @mention notifications |
| `notif-like` | Like/reaction notifications |
| `notif-follow` | New follower notifications |
| `notif-reply` | Reply notifications |
| `notif-system` | System notifications |

## Best Practices

1. **Contrast**: Ensure text is readable against backgrounds (WCAG AA minimum)
2. **Consistency**: Keep related colors cohesive (e.g., all error states similar)
3. **Accent Usage**: Use accent sparingly for emphasis
4. **Test Both Modes**: If creating both user & synapse themes, test them together

## Color Tips

- **Dark themes**: Window background 10-15% lighter than pure black for depth
- **Light themes**: Use off-white (#f8fafc) instead of pure white
- **Accent colors**: 60-70% saturation works well for most accents
- **Muted variants**: Use 10-15% opacity of the base color
