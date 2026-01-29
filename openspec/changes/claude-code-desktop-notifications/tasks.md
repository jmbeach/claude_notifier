## 1. Project Setup

- [x] 1.1 Add serde and serde_json dependencies to Cargo.toml
- [x] 1.2 Add notify-rust dependency to Cargo.toml

## 2. Hook Context Parsing

- [x] 2.1 Define HookContext struct with hook_type field and optional message
- [x] 2.2 Implement stdin reading and JSON deserialization
- [x] 2.3 Handle parse errors with non-zero exit code

## 3. Hook Type Handling

- [x] 3.1 Match on hook type to determine action
- [x] 3.2 Handle Stop hook - trigger notification with "waiting for input" message
- [x] 3.3 Handle Notification hook - trigger notification with hook's message
- [x] 3.4 Ignore other hook types - exit successfully without notification

## 4. Desktop Notifications

- [x] 4.1 Create notification function using notify-rust
- [x] 4.2 Set notification title to "Claude Code"
- [x] 4.3 Set notification body based on hook type
- [x] 4.4 Handle notification errors gracefully with stderr output

## 5. Testing & Documentation

- [x] 5.1 Test with sample Stop hook JSON
- [x] 5.2 Test with sample Notification hook JSON
- [x] 5.3 Document Claude Code hooks configuration in README
