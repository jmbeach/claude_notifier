### Requirement: Send native desktop notification
The system SHALL send a native desktop notification using the platform's notification system.

#### Scenario: Send notification on Windows
- **WHEN** a notification is triggered on Windows
- **THEN** the system displays a Windows toast notification

#### Scenario: Send notification on macOS
- **WHEN** a notification is triggered on macOS
- **THEN** the system displays a macOS Notification Center notification

#### Scenario: Send notification on Linux
- **WHEN** a notification is triggered on Linux with libnotify available
- **THEN** the system displays a notification via libnotify/D-Bus

### Requirement: Notification contains title and body
The system SHALL display notifications with a title and body text.

#### Scenario: Stop notification content
- **WHEN** a Stop hook triggers a notification
- **THEN** the notification title is "Claude Code" and body indicates waiting for input

#### Scenario: Notification hook content
- **WHEN** a Notification hook triggers a notification
- **THEN** the notification title is "Claude Code" and body contains the hook's message

### Requirement: Graceful failure on notification error
The system SHALL exit gracefully if the notification cannot be sent.

#### Scenario: Notification system unavailable
- **WHEN** the platform notification system is unavailable
- **THEN** the system logs an error to stderr and exits with non-zero status
