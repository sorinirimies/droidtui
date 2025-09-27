use crate::{
    effects::EffectsManager,
    event::{AppEvent, Event, EventHandler},
    menu::Menu,
};
use ratatui::{
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
    DefaultTerminal,
};
use std::time::Instant;
use tokio::process::Command as AsyncCommand;

/// Application state
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AppState {
    Startup,
    Menu,
    Executing,
    Loading,
    ShowResult,
}

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// Current application state
    pub state: AppState,
    /// Main menu
    pub menu: Menu,
    /// Effects manager for animations
    pub effects: EffectsManager,
    /// Event handler
    pub events: EventHandler,
    /// Last tick time for effects
    pub last_tick: Instant,
    /// Command execution result
    pub command_result: Option<String>,
    /// Command execution error
    pub command_error: Option<String>,
    /// Loading animation counter
    pub loading_counter: u64,
    /// Scroll position for command results
    pub scroll_position: usize,
    /// Total lines in command result
    pub result_lines: Vec<String>,
    /// Wrapped lines for display (handles wide content)
    pub wrapped_lines: Vec<String>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            state: AppState::Startup,
            menu: Menu::new(),
            effects: EffectsManager::new(),
            events: EventHandler::new(),
            last_tick: Instant::now(),
            command_result: None,
            command_error: None,
            loading_counter: 0,
            scroll_position: 0,
            result_lines: Vec::new(),
            wrapped_lines: Vec::new(),
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Run the application's main loop.
    pub async fn run(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        while self.running {
            terminal.draw(|frame| frame.render_widget(&mut self, frame.area()))?;
            match self.events.next().await? {
                Event::Tick => self.tick(),
                Event::Crossterm(event) => {
                    if let crossterm::event::Event::Key(key_event) = event {
                        self.handle_key_events(key_event)?
                    }
                }
                Event::App(app_event) => match app_event {
                    AppEvent::MenuUp => self.menu_up(),
                    AppEvent::MenuDown => self.menu_down(),
                    AppEvent::Execute => self.execute_selected().await,
                    AppEvent::EnterChild => self.enter_child_mode().await,
                    AppEvent::ExitChild => self.exit_child_mode(),
                    AppEvent::Quit => self.quit(),
                },
            }
        }
        Ok(())
    }

    /// Handles the key events and updates the state of [`App`].
    pub fn handle_key_events(&mut self, key_event: KeyEvent) -> color_eyre::Result<()> {
        match self.state {
            AppState::Startup => {
                // Skip startup animation on any key press
                self.state = AppState::Menu;
            }
            AppState::Menu => match key_event.code {
                KeyCode::Esc | KeyCode::Char('q') => self.events.send(AppEvent::Quit),
                KeyCode::Char('c' | 'C') if key_event.modifiers == KeyModifiers::CONTROL => {
                    self.events.send(AppEvent::Quit)
                }
                KeyCode::Up | KeyCode::Char('k') => self.events.send(AppEvent::MenuUp),
                KeyCode::Down | KeyCode::Char('j') => self.events.send(AppEvent::MenuDown),
                KeyCode::Enter => {
                    if self.menu.is_in_child_mode() {
                        self.events.send(AppEvent::Execute)
                    } else {
                        self.events.send(AppEvent::EnterChild)
                    }
                }
                KeyCode::Right => {
                    if !self.menu.is_in_child_mode() {
                        self.events.send(AppEvent::EnterChild)
                    }
                }
                KeyCode::Left | KeyCode::Backspace => {
                    if self.menu.is_in_child_mode() {
                        self.events.send(AppEvent::ExitChild)
                    }
                }
                _ => {}
            },
            AppState::Executing | AppState::Loading => {
                // Allow quitting during execution or loading
                match key_event.code {
                    KeyCode::Esc | KeyCode::Char('q') => {
                        self.state = AppState::Menu;
                        self.command_result = None;
                        self.command_error = None;
                    }
                    _ => {}
                }
            }
            AppState::ShowResult => {
                // Handle scrolling and return to menu
                match key_event.code {
                    KeyCode::Up | KeyCode::Char('k') => {
                        if self.scroll_position > 0 {
                            self.scroll_position -= 1;
                        }
                    }
                    KeyCode::Down | KeyCode::Char('j') => {
                        if self.scroll_position + 1 < self.wrapped_lines.len() {
                            self.scroll_position += 1;
                        }
                    }
                    KeyCode::PageUp => {
                        self.scroll_position = self.scroll_position.saturating_sub(10);
                    }
                    KeyCode::PageDown => {
                        self.scroll_position = (self.scroll_position + 10)
                            .min(self.wrapped_lines.len().saturating_sub(1));
                    }
                    KeyCode::Home => {
                        self.scroll_position = 0;
                    }
                    KeyCode::End => {
                        self.scroll_position = self.wrapped_lines.len().saturating_sub(1);
                    }
                    KeyCode::Esc | KeyCode::Char('q') | KeyCode::Enter | KeyCode::Backspace => {
                        self.state = AppState::Menu;
                        self.command_result = None;
                        self.command_error = None;
                        self.scroll_position = 0;
                        self.result_lines.clear();
                        self.wrapped_lines.clear();
                    }
                    _ => {
                        // For other keys, return to menu
                        self.state = AppState::Menu;
                        self.command_result = None;
                        self.command_error = None;
                        self.scroll_position = 0;
                        self.result_lines.clear();
                        self.wrapped_lines.clear();
                    }
                }
            }
        }
        Ok(())
    }

    /// Handles the tick event of the terminal.
    ///
    /// The tick event is where you can update the state of your application with any logic that
    /// needs to be updated at a fixed frame rate. E.g. polling a server, updating an animation.
    pub fn tick(&mut self) {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_tick);
        self.last_tick = now;

        // Update effects
        self.effects.tick(elapsed);

        // Update menu animations
        self.menu.tick();

        // Update loading animation
        if self.state == AppState::Loading {
            self.loading_counter += 1;
        }

        // Check if startup is complete
        if self.state == AppState::Startup && self.effects.is_startup_complete() {
            self.state = AppState::Menu;
        }
    }

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn menu_up(&mut self) {
        self.menu.previous();
    }

    pub fn menu_down(&mut self) {
        self.menu.next();
    }

    pub async fn execute_selected(&mut self) {
        if let Some(command) = self.menu.get_selected_command() {
            self.state = AppState::Loading;
            self.command_result = None;
            self.command_error = None;
            self.loading_counter = 0;

            // Execute the command asynchronously
            let command_parts: Vec<&str> = command.split_whitespace().collect();
            if !command_parts.is_empty() {
                let result = {
                    let mut cmd = AsyncCommand::new(command_parts[0]);
                    if command_parts.len() > 1 {
                        cmd.args(&command_parts[1..]);
                    }
                    cmd.output().await
                };

                match result {
                    Ok(output) => {
                        let stdout = String::from_utf8_lossy(&output.stdout);
                        let stderr = String::from_utf8_lossy(&output.stderr);

                        if output.status.success() {
                            let result_text =
                                if stdout.trim().is_empty() && !stderr.trim().is_empty() {
                                    // Some commands output to stderr even on success
                                    stderr.to_string()
                                } else if stdout.trim().is_empty() {
                                    "Command executed successfully (no output)".to_string()
                                } else {
                                    stdout.to_string()
                                };
                            self.command_result = Some(result_text.clone());
                            self.result_lines =
                                result_text.lines().map(|s| s.to_string()).collect();
                            self.update_wrapped_lines();
                        } else {
                            let error_msg = if stderr.trim().is_empty() {
                                format!("Command failed with exit code: {}", output.status)
                            } else {
                                stderr.to_string()
                            };
                            self.command_error = Some(error_msg.clone());
                            self.result_lines = error_msg.lines().map(|s| s.to_string()).collect();
                            self.update_wrapped_lines();
                        }
                        self.scroll_position = 0;
                    }
                    Err(e) => {
                        let error_msg = match e.kind() {
                            std::io::ErrorKind::NotFound => {
                                if command_parts[0] == "adb" {
                                    "ADB not found. Please install Android SDK tools and add ADB to your PATH.".to_string()
                                } else {
                                    format!("Command '{}' not found", command_parts[0])
                                }
                            }
                            std::io::ErrorKind::PermissionDenied => {
                                "Permission denied. You may need to run with elevated privileges."
                                    .to_string()
                            }
                            _ => format!("Failed to execute command: {}", e),
                        };
                        self.command_error = Some(error_msg.clone());
                        self.result_lines = error_msg.lines().map(|s| s.to_string()).collect();
                        self.update_wrapped_lines();
                        self.scroll_position = 0;
                    }
                }
            } else {
                let error_msg = "Invalid command".to_string();
                self.command_error = Some(error_msg.clone());
                self.result_lines = error_msg.lines().map(|s| s.to_string()).collect();
                self.update_wrapped_lines();
                self.scroll_position = 0;
            }

            self.state = AppState::ShowResult;
        }
    }

    pub async fn enter_child_mode(&mut self) {
        self.menu.enter_child_mode();
        // Trigger fade-in effect
        self.effects.start_fade_in();
        // Small delay to show fade-in animation
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
    }

    pub fn exit_child_mode(&mut self) {
        self.menu.exit_child_mode();
    }

    /// Update wrapped lines for display based on terminal width
    pub fn update_wrapped_lines(&mut self) {
        // Use a reasonable default width, will be updated in UI rendering
        let max_width = 80;
        self.wrapped_lines = if self.result_lines.is_empty() {
            vec!["No output".to_string()]
        } else {
            self.result_lines
                .iter()
                .flat_map(|line| {
                    if line.len() <= max_width {
                        vec![line.clone()]
                    } else {
                        // Break long lines into chunks
                        line.chars()
                            .collect::<Vec<char>>()
                            .chunks(max_width)
                            .map(|chunk| chunk.iter().collect::<String>())
                            .collect()
                    }
                })
                .collect()
        };
    }
}
