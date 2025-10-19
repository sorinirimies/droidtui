use crate::adb::AdbCommand;
use crate::message::{CommandResult, Message};
use crate::model::{AppState, Model};
use crate::stream::{start_stream, StreamConfig};

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
            model.effects.start_slide_in();
        }

        Message::ExitChild => {
            model.menu.exit_child_mode();
        }

        // Command execution
        Message::ExecuteCommand(command) => {
            // Check if this is a stream command (legacy support)
            if let AdbCommand::Shell { command: cmd } = &command {
                if cmd.starts_with("STREAM") {
                    // Configure based on stream type
                    let config = match cmd.as_str() {
                        "STREAM_HD" => StreamConfig {
                            width: 1080,
                            height: 1920,
                            bitrate: "12M".to_string(),
                        },
                        "STREAM_FAST" => StreamConfig {
                            width: 720,
                            height: 1280,
                            bitrate: "4M".to_string(),
                        },
                        _ => StreamConfig::default(),
                    };

                    // Launch streaming in separate window (non-blocking)
                    match start_stream(config) {
                        Ok(stream_state) => {
                            model.stream_state = Some(stream_state);
                            model.set_result("Screen streaming started in separate window.\nClose the window or press Q in it to stop.".to_string());
                            model.state = AppState::ShowResult;
                        }
                        Err(e) => {
                            model.set_error(format!("Failed to start streaming: {}\n\nMake sure:\n- ADB is installed and in PATH\n- Device is connected\n- FFmpeg is installed", e));
                            model.state = AppState::ShowResult;
                        }
                    }
                    model.effects.start_slide_in();
                    return;
                }
            }

            model.state = AppState::Loading;
            model.clear_results();
            model.loading_counter = 0;
            model.effects.reset_slide();

            let result = execute_adb_command(model, command).await;

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
            model.effects.start_slide_in();
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
            model.effects.start_slide_in();
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

        // Screen streaming messages
        Message::StartStream => {
            let config = StreamConfig::default();
            match start_stream(config) {
                Ok(stream_state) => {
                    model.stream_state = Some(stream_state);
                    model.set_result("Screen streaming started in separate window.".to_string());
                    model.state = AppState::ShowResult;
                }
                Err(e) => {
                    model.set_error(format!("Failed to start streaming: {}", e));
                    model.state = AppState::ShowResult;
                }
            }
            model.effects.start_slide_in();
        }

        Message::StopStream => {
            if let Some(mut stream) = model.stream_state.take() {
                stream.stop();
            }
            model.state = AppState::Menu;
        }

        Message::UpdateFrame(_frame) => {
            // No longer used with window-based streaming
        }

        Message::TogglePause => {
            // No longer used with window-based streaming
        }

        Message::IncreaseRefreshRate => {
            // No longer used with window-based streaming
        }

        Message::DecreaseRefreshRate => {
            // No longer used with window-based streaming
        }

        // Application lifecycle
        Message::Tick => {
            tick(model).await;
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
            model.effects.start_slide_in();
        }
    }
}

/// Handle tick updates (animations, timers, etc.)
async fn tick(model: &mut Model) {
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

    // Update reveal animation for results
    if model.state == AppState::ShowResult {
        model.reveal_counter += 1;
    }

    // Check if startup is complete
    if model.state == AppState::Startup && model.effects.is_startup_complete() {
        model.state = AppState::Menu;
    }
}

/// Execute an ADB command using the ADB manager
async fn execute_adb_command(model: &mut Model, command: AdbCommand) -> CommandResult {
    // Execute the command using the ADB manager
    match model.adb_manager.execute(command) {
        Ok(output) => CommandResult::Success(output),
        Err(e) => {
            let error_msg = format!("{}", e);

            // Add helpful context for common errors
            let enhanced_error = if error_msg.contains("Connection")
                || error_msg.contains("connection")
            {
                format!(
                    "{}\n\nTroubleshooting:\n• Make sure ADB server is running (adb start-server)\n• Check that device is connected (adb devices)\n• Verify USB debugging is enabled on device",
                    error_msg
                )
            } else if error_msg.contains("No device selected") {
                format!(
                    "{}\n\nPlease:\n• Connect an Android device via USB\n• Enable USB debugging on the device\n• Run 'List Devices' to detect your device",
                    error_msg
                )
            } else {
                error_msg
            };

            CommandResult::Error(enhanced_error)
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

    #[tokio::test]
    async fn test_enter_exit_child_mode() {
        let mut model = Model::new();
        model.state = AppState::Menu;

        update(&mut model, Message::EnterChild).await;
        assert!(model.menu.is_in_child_mode());

        update(&mut model, Message::ExitChild).await;
        assert!(!model.menu.is_in_child_mode());
    }

    #[tokio::test]
    async fn test_clear_results() {
        let mut model = Model::new();
        model.set_result("Test output".to_string());
        assert!(model.command_result.is_some());
        assert!(!model.result_lines.is_empty());

        update(&mut model, Message::ReturnToMenu).await;
        assert!(model.command_result.is_none());
        assert!(model.result_lines.is_empty());
        assert_eq!(model.state, AppState::Menu);
    }
}
