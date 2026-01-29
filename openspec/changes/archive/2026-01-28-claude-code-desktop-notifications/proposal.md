## Why

Claude Code runs in the terminal and provides no notification when it finishes a response or needs user input. Users must constantly monitor the terminal to know when Claude is ready, which interrupts focus and wastes attention.

## What Changes

- Create a CLI tool designed to be invoked by Claude Code hooks
- Parse hook context (JSON from stdin) to determine notification content
- Send native desktop notifications when Claude completes a response or awaits input

## Capabilities

### New Capabilities
- `hook-handler`: Parse Claude Code hook context from stdin and determine appropriate notification based on hook type (Stop, etc.)
- `desktop-notification`: Send cross-platform native desktop notifications (Windows toast, macOS Notification Center, Linux libnotify)

### Modified Capabilities
(none - this is a greenfield project)

## Impact

- **Dependencies**: Will need platform-specific notification libraries (notify-rust or similar)
- **Platform support**: Windows, macOS, Linux desktop environments
- **Integration**: Invoked exclusively via Claude Code hooks configuration
