### Requirement: Parse hook context from stdin
The system SHALL read JSON from stdin and parse it as Claude Code hook context.

#### Scenario: Valid JSON input
- **WHEN** valid Claude Code hook JSON is piped to stdin
- **THEN** the system parses the JSON and extracts the hook type and relevant fields

#### Scenario: Invalid JSON input
- **WHEN** invalid JSON is piped to stdin
- **THEN** the system exits with a non-zero status code

#### Scenario: Empty stdin
- **WHEN** no input is provided to stdin
- **THEN** the system exits with a non-zero status code

### Requirement: Trigger notification on Stop hook
The system SHALL send a desktop notification when the hook type is "Stop".

#### Scenario: Stop hook received
- **WHEN** a Stop hook context is received
- **THEN** the system sends a notification indicating Claude is waiting for input

### Requirement: Trigger notification on Notification hook
The system SHALL send a desktop notification when the hook type is "Notification".

#### Scenario: Notification hook received
- **WHEN** a Notification hook context is received with a message
- **THEN** the system sends a notification containing the provided message

### Requirement: Ignore other hook types
The system SHALL silently exit without notification for unhandled hook types.

#### Scenario: PreToolUse hook received
- **WHEN** a PreToolUse hook context is received
- **THEN** the system exits successfully without sending a notification

#### Scenario: PostToolUse hook received
- **WHEN** a PostToolUse hook context is received
- **THEN** the system exits successfully without sending a notification
