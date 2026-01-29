## Context

Claude Code provides a hooks system that executes shell commands on specific events. This tool will be a Rust binary invoked by these hooks, receiving event context via stdin as JSON. The binary must be lightweight and fast since it runs on every hook event.

## Goals / Non-Goals

**Goals:**
- Send desktop notifications when Claude Code completes a response or needs input
- Parse Claude Code hook JSON from stdin to determine event type
- Support Windows, macOS, and Linux notification systems
- Fast startup and execution (hooks should not slow down Claude Code)

**Non-Goals:**
- Configuration UI or interactive mode
- Notification history or logging
- Custom notification sounds or icons (use system defaults)
- Running as a daemon or background service

## Decisions

### 1. Use `notify-rust` crate for cross-platform notifications
**Rationale**: Mature, well-maintained crate that abstracts platform differences. Supports Windows toast notifications, macOS Notification Center, and Linux libnotify/D-Bus.
**Alternatives considered**:
- Platform-specific code: More control but significant maintenance burden
- `alert` crate: Less mature, fewer features

### 2. Read hook context from stdin as JSON
**Rationale**: Claude Code hooks pass context via stdin. Using serde_json for parsing is idiomatic Rust and handles the structured data well.
**Alternatives considered**:
- Command-line arguments: Limited by shell escaping issues with complex data
- Environment variables: Size limits and escaping concerns

### 3. Single binary with no subcommands
**Rationale**: The tool has one job—receive hook context, send notification. No need for CLI complexity. Just run `claude_notifier` and pipe JSON to it.
**Alternatives considered**:
- Subcommands (e.g., `claude_notifier notify`): Unnecessary indirection for single-purpose tool

### 4. Handle `Stop` and `Notification` hooks
**Rationale**: 
- `Stop` hook fires when Claude completes a response and is waiting for user input
- `Notification` hook fires when Claude explicitly wants to notify the user (e.g., long-running task complete)

Both are appropriate triggers for desktop notifications. Other hooks (PreToolUse, PostToolUse) would create noise.
**Alternatives considered**:
- Notify on all events: Too noisy, defeats purpose
- Only `Stop`: Would miss explicit notification requests from Claude

## Risks / Trade-offs

**[Risk] Notification library may not work on all Linux distros** → Mitigation: Document required dependencies (libnotify/D-Bus). Fail gracefully with error message if notification fails.

**[Risk] Fast repeated notifications could be annoying** → Mitigation: Consider debouncing in future iteration if needed. For now, `Stop` events are infrequent enough.

**[Trade-off] No configuration** → Keeps tool simple but limits customization. Can add config file support later if needed.
