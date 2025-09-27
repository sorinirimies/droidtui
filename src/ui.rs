use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, BorderType, Paragraph, Widget},
};

use crate::{
    app::{App, AppState},
    effects::{RevealWidget, get_loading_dots, get_loading_spinner, get_progress_bar},
};

impl Widget for &mut App {
    /// Renders the user interface widgets.
    fn render(self, area: Rect, buf: &mut Buffer) {
        match self.state {
            AppState::Startup => {
                self.render_startup(area, buf);
            }
            AppState::Menu => {
                self.render_menu(area, buf);
            }
            AppState::Loading => {
                self.render_loading(area, buf);
            }
            AppState::Executing => {
                self.render_executing(area, buf);
            }
            AppState::ShowResult => {
                self.render_result(area, buf);
            }
        }
    }
}

impl App {
    fn render_startup(&mut self, area: Rect, buf: &mut Buffer) {
        // Render reveal animation
        let reveal_widget = RevealWidget::new(
            &mut self.effects,
            "🚀 DroidTUI - Android Development Toolkit",
            "Your powerful ADB command center with visual effects",
        );
        reveal_widget.render(area, buf);
    }

    fn render_loading(&mut self, area: Rect, buf: &mut Buffer) {
        // Create animated loading screen
        let spinner = get_loading_spinner(self.loading_counter);
        let dots = get_loading_dots(self.loading_counter);
        let progress = get_progress_bar(self.loading_counter, 30);

        let loading_text = format!(
            "{} Executing Command {}\n\n{}\n\nPress Esc to cancel",
            spinner, dots, progress
        );

        let loading_block = Block::bordered()
            .title("⚡ Processing")
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Rounded)
            .style(Style::default().fg(Color::Yellow));

        let loading_paragraph = Paragraph::new(loading_text)
            .block(loading_block)
            .style(Style::default().fg(Color::LightYellow))
            .alignment(Alignment::Center);

        // Center the loading dialog
        let popup_area = centered_rect(50, 30, area);
        loading_paragraph.render(popup_area, buf);
    }

    fn render_menu(&mut self, area: Rect, buf: &mut Buffer) {
        // Create layout
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Header
                Constraint::Min(0),    // Menu content
                Constraint::Length(3), // Footer
            ])
            .split(area);

        // Render header
        let header_block = Block::bordered()
            .title("🤖 DroidTUI - Android Development Toolkit")
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Rounded)
            .style(Style::default().fg(Color::Green));

        let header = Paragraph::new("ADB Command Interface")
            .block(header_block)
            .style(Style::default().fg(Color::LightGreen))
            .alignment(Alignment::Center);

        header.render(chunks[0], buf);

        // Render menu
        (&self.menu).render(chunks[1], buf);

        // Render footer with help
        let footer_block = Block::bordered()
            .title("Help")
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Rounded)
            .style(Style::default().fg(Color::Yellow));

        let footer_text = if self.menu.is_in_child_mode() {
            "↑/↓ or j/k: Navigate | Enter: Execute | ← Left/Backspace: Back | q/Esc: Quit"
        } else {
            "↑/↓ or j/k: Navigate | Enter/→: Options | q/Esc: Quit"
        };

        // Apply fade-in effect if entering child mode
        if self.menu.is_in_child_mode() && !self.effects.is_fade_in_complete() {
            let fade_progress = self.effects.get_fade_in_progress();

            // Apply fade effect to the entire menu area
            for y in area.top()..area.bottom() {
                for x in area.left()..area.right() {
                    if let Some(cell) = buf.cell_mut((x, y)) {
                        // Simple fade effect - adjust alpha by modifying colors
                        match cell.fg {
                            Color::Green => {
                                let intensity = (255.0 * fade_progress) as u8;
                                cell.set_fg(Color::Rgb(0, intensity.min(255), 0));
                            }
                            Color::White => {
                                let intensity = (255.0 * fade_progress) as u8;
                                cell.set_fg(Color::Rgb(intensity, intensity, intensity));
                            }
                            Color::Yellow => {
                                let intensity = (255.0 * fade_progress) as u8;
                                cell.set_fg(Color::Rgb(intensity, intensity, 0));
                            }
                            _ => {
                                // For other colors, apply a general fade
                                if fade_progress < 0.5 {
                                    cell.set_fg(Color::DarkGray);
                                }
                            }
                        }
                    }
                }
            }
        }

        let footer = Paragraph::new(footer_text)
            .block(footer_block)
            .style(Style::default().fg(Color::White))
            .alignment(Alignment::Center);

        footer.render(chunks[2], buf);
    }

    fn render_executing(&mut self, area: Rect, buf: &mut Buffer) {
        let executing_text = if let Some(command) = self.menu.get_selected_command() {
            format!(
                "Executing command...\n\nCommand: {}\n\nPress Esc to return to menu",
                command
            )
        } else {
            "Executing command...\n\nPress Esc to return to menu".to_string()
        };

        let executing_block = Block::bordered()
            .title("⚡ Executing Command")
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Rounded)
            .style(Style::default().fg(Color::Yellow));

        let executing_paragraph = Paragraph::new(executing_text)
            .block(executing_block)
            .style(Style::default().fg(Color::White))
            .alignment(Alignment::Center);

        // Center the executing dialog
        let popup_area = centered_rect(70, 50, area);
        executing_paragraph.render(popup_area, buf);
    }

    fn render_result(&mut self, area: Rect, buf: &mut Buffer) {
        let (title, color) = if self.command_result.is_some() {
            ("✅ Command Result", Color::Green)
        } else if self.command_error.is_some() {
            ("❌ Command Error", Color::Red)
        } else {
            ("📋 No Output", Color::Yellow)
        };

        // Use larger area for results to accommodate more text
        let popup_area = centered_rect(80, 70, area);

        // Split area for content and scroll bar
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Min(0),    // Content area
                Constraint::Length(3), // Scroll bar area
            ])
            .split(popup_area);

        let content_area = chunks[0];
        let scrollbar_area = chunks[1];

        // Calculate available height for content (subtract borders and help text)
        let content_height = content_area.height.saturating_sub(4); // 2 for borders, 2 for help text
        let visible_lines = content_height as usize;

        // Update wrapped lines with current terminal width
        let max_width = content_area.width.saturating_sub(4) as usize; // Account for borders
        if self.result_lines.is_empty() {
            self.wrapped_lines = vec!["No output".to_string()];
        } else {
            self.wrapped_lines = self
                .result_lines
                .iter()
                .flat_map(|line| {
                    if line.len() <= max_width {
                        vec![line.clone()]
                    } else {
                        // Break long lines into chunks at word boundaries when possible
                        let mut chunks = Vec::new();
                        let mut current_chunk = String::new();
                        let mut current_length = 0;

                        for word in line.split_whitespace() {
                            if current_length + word.len() + 1 <= max_width {
                                if !current_chunk.is_empty() {
                                    current_chunk.push(' ');
                                    current_length += 1;
                                }
                                current_chunk.push_str(word);
                                current_length += word.len();
                            } else {
                                if !current_chunk.is_empty() {
                                    chunks.push(current_chunk);
                                }
                                if word.len() > max_width {
                                    // Word is too long, break it
                                    for chunk in
                                        word.chars().collect::<Vec<char>>().chunks(max_width)
                                    {
                                        chunks.push(chunk.iter().collect::<String>());
                                    }
                                    current_chunk = String::new();
                                    current_length = 0;
                                } else {
                                    current_chunk = word.to_string();
                                    current_length = word.len();
                                }
                            }
                        }
                        if !current_chunk.is_empty() {
                            chunks.push(current_chunk);
                        }
                        chunks
                    }
                })
                .collect();
        }

        // Determine visible content based on scroll position
        let total_lines = self.wrapped_lines.len();
        let start_line = self.scroll_position;
        let end_line = (start_line + visible_lines).min(total_lines);

        let visible_content: Vec<String> = self.wrapped_lines[start_line..end_line].to_vec();

        // Create scroll indicator
        let scroll_info = if total_lines > visible_lines {
            format!(
                " [{}/{}] ↑/↓:Scroll PgUp/PgDn:Fast Home/End:Jump",
                start_line + 1,
                total_lines
            )
        } else {
            " Press Esc/q/Enter to return".to_string()
        };

        // Create title with scroll info
        let full_title = format!("{}{}", title, scroll_info);

        let result_block = Block::bordered()
            .title(full_title)
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Rounded)
            .style(Style::default().fg(color));

        // Join visible lines and add help text at bottom
        let mut display_text = visible_content.join("\n");

        // Add scroll indicator if there's more content
        if total_lines > visible_lines {
            let padding = "\n".repeat((visible_lines.saturating_sub(visible_content.len())).max(1));
            display_text.push_str(&padding);

            if start_line > 0 {
                display_text.push_str("▲ More content above");
            }
            if end_line < total_lines {
                if start_line > 0 {
                    display_text.push_str(" | ");
                }
                display_text.push_str("▼ More content below");
            }
        } else if total_lines <= visible_lines && total_lines > 0 {
            let padding = "\n".repeat((visible_lines.saturating_sub(visible_content.len())).max(1));
            display_text.push_str(&padding);
            display_text.push_str("Press Esc/q/Enter to return to menu");
        }

        let result_paragraph = Paragraph::new(display_text)
            .block(result_block)
            .style(Style::default().fg(Color::White))
            .alignment(Alignment::Left)
            .wrap(ratatui::widgets::Wrap { trim: false });

        result_paragraph.render(content_area, buf);

        // Render scroll bar if content is scrollable
        if total_lines > visible_lines {
            self.render_scrollbar(
                scrollbar_area,
                buf,
                total_lines,
                visible_lines,
                start_line,
                color,
            );
        }
    }

    fn render_scrollbar(
        &self,
        area: Rect,
        buf: &mut Buffer,
        total_lines: usize,
        visible_lines: usize,
        scroll_pos: usize,
        color: Color,
    ) {
        if area.height < 3 {
            return; // Not enough space for scrollbar
        }

        // Create scrollbar block
        let scrollbar_block = Block::bordered()
            .border_type(BorderType::Rounded)
            .style(Style::default().fg(color));

        // Calculate inner area before rendering
        let inner_area = scrollbar_block.inner(area);

        scrollbar_block.render(area, buf);
        let scrollbar_height = inner_area.height as usize;

        if scrollbar_height == 0 {
            return;
        }

        // Calculate thumb position and size
        let thumb_size = ((visible_lines as f64 / total_lines as f64) * scrollbar_height as f64)
            .max(1.0) as usize;
        let thumb_pos = ((scroll_pos as f64 / (total_lines - visible_lines) as f64)
            * (scrollbar_height - thumb_size) as f64) as usize;

        // Render scrollbar track and thumb
        for y in 0..scrollbar_height {
            let cell_y = inner_area.y + y as u16;
            let cell_x = inner_area.x;

            if let Some(cell) = buf.cell_mut((cell_x, cell_y)) {
                if y >= thumb_pos && y < thumb_pos + thumb_size {
                    // Render thumb
                    cell.set_char('█');
                    cell.set_fg(color);
                } else {
                    // Render track
                    cell.set_char('░');
                    cell.set_fg(Color::DarkGray);
                }
            }
        }
    }
}

/// helper function to create a centered rect using up certain percentage of the available rect `r`
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
