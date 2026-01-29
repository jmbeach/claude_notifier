# Claude Notifier

Desktop notifications for Claude Code. Get notified when Claude finishes a response or needs your input.

## Installation

```bash
cargo install --path .
```

## Claude Code Hooks Configuration

Add to your Claude Code settings (`.claude/settings.json` or global settings):

```json
{
  "hooks": {
    "Stop": [
      {
        "command": "claude_notifier"
      }
    ],
    "Notification": [
      {
        "command": "claude_notifier"
      }
    ]
  }
}
```

## Supported Hooks

- **Stop**: Triggered when Claude completes a response and is waiting for input
- **Notification**: Triggered when Claude explicitly wants to notify you

## Platform Support

- **Windows**: Toast notifications
- **macOS**: Notification Center
- **Linux**: libnotify/D-Bus (requires notification daemon)

## Requirements

### Linux
Requires a notification daemon (e.g., `dunst`, `mako`, or desktop environment notifications).
