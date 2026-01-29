use notify_rust::Notification;
use serde::Deserialize;
use std::io::{self, Read};
use std::process::ExitCode;

#[derive(Deserialize)]
struct HookContext {
    hook_type: String,
    message: Option<String>,
}

fn send_notification(title: &str, body: &str) -> Result<(), notify_rust::error::Error> {
    Notification::new()
        .summary(title)
        .body(body)
        .show()?;
    Ok(())
}

fn main() -> ExitCode {
    let mut input = String::new();
    if let Err(e) = io::stdin().read_to_string(&mut input) {
        eprintln!("Failed to read stdin: {}", e);
        return ExitCode::FAILURE;
    }

    if input.is_empty() {
        eprintln!("No input provided");
        return ExitCode::FAILURE;
    }

    let context: HookContext = match serde_json::from_str(&input) {
        Ok(ctx) => ctx,
        Err(e) => {
            eprintln!("Failed to parse JSON: {}", e);
            return ExitCode::FAILURE;
        }
    };

    let notification_body = match context.hook_type.as_str() {
        "Stop" => Some("Claude is waiting for your input".to_string()),
        "Notification" => context.message,
        _ => None,
    };

    if let Some(body) = notification_body {
        if let Err(e) = send_notification("Claude Code", &body) {
            eprintln!("Failed to send notification: {}", e);
            return ExitCode::FAILURE;
        }
    }

    ExitCode::SUCCESS
}
