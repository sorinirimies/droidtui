use crate::message::{CommandResult, Message};
use crate::model::{AppState, Model};
use tokio::process::Command as AsyncCommand;

/// Update function - the heart of Elm architecture
/// Takes the current model and a message, returns updated model
/// This is a pure function that handles all state transitions
pub async fn update(model: &mut Model, message: Message) {
    match message {
        // Navigation messages
        Message::MenuUp => {
            model.menu.previous();
        }

        Message::MenuDown => {
            model.menu.next();
        }

        Message::EnterChild => {
            model.menu.enter_child_mode();
            model.effects.start_fade_in();
        }

        Message::ExitChild => {
            model.menu.exit_child_mode();
        }

        // Command execution
        Message::ExecuteCommand(command) => {
            model.state = AppState::Loading;
            model.clear_results();
            model.loading_counter = 0;

            let result = execute_command(&command).await;

            // Handle result directly to avoid recursion
            match result {
                CommandResult::Success(output) => {
                    model.set_result(output);
                }
                CommandResult::Error(error) => {
                    model.set_error(error);
                }
            }
            model.state = AppState::ShowResult;
        }

        Message::CommandStarted => {
            model.state = AppState::Loading;
            model.loading_counter = 0;
        }

        Message::CommandCompleted(result) => {
            match result {
                CommandResult::Success(output) => {
                    model.set_result(output);
                }
                CommandResult::Error(error) => {
                    model.set_error(error);
                }
            }
            model.state = AppState::ShowResult;
        }

        // Scroll messages
        Message::ScrollUp => {
            if model.scroll_position > 0 {
                model.scroll_position -= 1;
            }
        }

        Message::ScrollDown => {
            if model.scroll_position + 1 < model.total_result_lines() {
                model.scroll_position += 1;
            }
        }

        Message::ScrollPageUp => {
            model.scroll_position = model.scroll_position.saturating_sub(10);
        }

        Message::ScrollPageDown => {
            let max_scroll = model.total_result_lines().saturating_sub(1);
            model.scroll_position = (model.scroll_position + 10).min(max_scroll);
        }

        Message::ScrollToTop => {
            model.scroll_position = 0;
        }

        Message::ScrollToBottom => {
            model.scroll_position = model.total_result_lines().saturating_sub(1);
        }

        // Application lifecycle
        Message::Tick => {
            tick(model);
        }

        Message::Quit => {
            model.running = false;
        }

        Message::ReturnToMenu => {
            model.state = AppState::Menu;
            model.clear_results();
        }

        Message::SkipStartup => {
            model.state = AppState::Menu;
        }
    }
}

/// Handle tick updates (animations, timers, etc.)
fn tick(model: &mut Model) {
    let now = std::time::Instant::now();
    let elapsed = now.duration_since(model.last_tick);
    model.last_tick = now;

    // Update effects
    model.effects.tick(elapsed);

    // Update menu animations
    model.menu.tick();

    // Update loading animation
    if model.state == AppState::Loading {
        model.loading_counter += 1;
    }

    // Check if startup is complete
    if model.state == AppState::Startup && model.effects.is_startup_complete() {
        model.state = AppState::Menu;
    }
}

/// Execute a shell command asynchronously
async fn execute_command(command: &str) -> CommandResult {
    let command_parts: Vec<&str> = command.split_whitespace().collect();

    if command_parts.is_empty() {
        return CommandResult::Error("Invalid command".to_string());
    }

    let mut cmd = AsyncCommand::new(command_parts[0]);
    if command_parts.len() > 1 {
        cmd.args(&command_parts[1..]);
    }

    match cmd.output().await {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);

            if output.status.success() {
                let result_text = if stdout.trim().is_empty() && !stderr.trim().is_empty() {
                    // Some commands output to stderr even on success
                    stderr.to_string()
                } else if stdout.trim().is_empty() {
                    "Command executed successfully (no output)".to_string()
                } else {
                    stdout.to_string()
                };
                CommandResult::Success(result_text)
            } else {
                let error_msg = if stderr.trim().is_empty() {
                    format!("Command failed with exit code: {}", output.status)
                } else {
                    stderr.to_string()
                };
                CommandResult::Error(error_msg)
            }
        }
        Err(e) => {
            let error_msg = match e.kind() {
                std::io::ErrorKind::NotFound => {
                    if command_parts[0] == "adb" {
                        "ADB not found. Please install Android SDK tools and add ADB to your PATH."
                            .to_string()
                    } else {
                        format!("Command '{}' not found", command_parts[0])
                    }
                }
                std::io::ErrorKind::PermissionDenied => {
                    "Permission denied. You may need to run with elevated privileges.".to_string()
                }
                _ => format!("Failed to execute command: {}", e),
            };
            CommandResult::Error(error_msg)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_menu_navigation() {
        let mut model = Model::new();
        model.state = AppState::Menu;

        update(&mut model, Message::MenuDown).await;
        assert_eq!(model.menu.selected, 1);

        update(&mut model, Message::MenuUp).await;
        assert_eq!(model.menu.selected, 0);
    }

    #[tokio::test]
    async fn test_quit() {
        let mut model = Model::new();
        assert!(model.running);

        update(&mut model, Message::Quit).await;
        assert!(!model.running);
    }

    #[tokio::test]
    async fn test_scroll_boundaries() {
        let mut model = Model::new();
        model.wrapped_lines = vec!["Line 1".to_string(), "Line 2".to_string()];
        model.scroll_position = 0;

        // Can't scroll up from position 0
        update(&mut model, Message::ScrollUp).await;
        assert_eq!(model.scroll_position, 0);

        // Can scroll down
        update(&mut model, Message::ScrollDown).await;
        assert_eq!(model.scroll_position, 1);

        // Can't scroll past end
        update(&mut model, Message::ScrollDown).await;
        assert_eq!(model.scroll_position, 1);
    }
}
