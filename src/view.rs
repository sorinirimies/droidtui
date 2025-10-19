use crate::effects::{
    get_dots_orbit, get_loading_dots, get_loading_spinner, get_orbital_spinner,
    get_particle_effect, get_progress_bar, get_wave_animation, RevealWidget,
};
use crate::model::{AppState, Model};
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, BorderType, Paragraph, Widget},
};

/// Main view function - renders the entire UI based on the model
/// This is pure function that takes model and produces UI
pub fn render(model: &mut Model, area: Rect, buf: &mut Buffer) {
    match model.state {
        AppState::Startup => render_startup(model, area, buf),
        AppState::Menu => render_menu(model, area, buf),
        AppState::Loading => render_loading(model, area, buf),
        AppState::Executing => render_executing(model, area, buf),
        AppState::ShowResult => render_result(model, area, buf),
    }
}

/// Render startup screen with animations
fn render_startup(model: &mut Model, area: Rect, buf: &mut Buffer) {
    let reveal_widget = RevealWidget::new(
        &mut model.effects,
        "🚀 DroidTUI - Android Development Toolkit",
        "Your powerful ADB command center with visual effects",
    );
    reveal_widget.render(area, buf);
}

/// Render loading screen with animations
fn render_loading(model: &Model, area: Rect, buf: &mut Buffer) {
    let spinner = get_loading_spinner(model.loading_counter);
    let dots = get_loading_dots(model.loading_counter);
    let progress = get_progress_bar(model.loading_counter, 30);

    // Create enhanced loading text with multiple spinners
    let orbital_spinner = get_orbital_spinner(model.loading_counter);
    let wave_animation = get_wave_animation(model.loading_counter);
    let dots_orbit = get_dots_orbit(model.loading_counter);
    let particle_effect = get_particle_effect(model.loading_counter);

    // Multi-layered loading display
    let loading_text = format!(
        "{} Executing Command {}\n\n{}\n\n{}\n\n{}\n{}\n\nPress Esc to cancel",
        spinner, dots, progress, wave_animation, dots_orbit, particle_effect
    );

    // Animate border color cycling between yellow and green
    let color_phase = (model.loading_counter as f32 * 0.05).sin() * 0.5 + 0.5;
    let border_color = if color_phase > 0.5 {
        Color::Yellow
    } else {
        Color::LightYellow
    };

    // Animated title with orbital spinner
    let animated_title = format!("⚡ Processing {} ", orbital_spinner);

    let loading_block = Block::bordered()
        .title(animated_title)
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Rounded)
        .style(Style::default().fg(border_color));

    // Cycle text color for shimmer effect
    let text_brightness = 170 + (color_phase * 85.0) as u8;
    let text_color = Color::Rgb(text_brightness, text_brightness, 0);

    let loading_paragraph = Paragraph::new(loading_text)
        .block(loading_block)
        .style(Style::default().fg(text_color))
        .alignment(Alignment::Center);

    // Create pulsing animation effect
    let pulse = (model.loading_counter as f32 * 0.1).sin() * 0.5 + 0.5;
    let scale_factor = 1.0 + (pulse * 0.1); // Pulse between 1.0 and 1.1

    let base_popup_area = centered_rect(55, 35, area);

    // Apply scale effect to the popup
    let scaled_width = (base_popup_area.width as f32 * scale_factor) as u16;
    let scaled_height = (base_popup_area.height as f32 * scale_factor) as u16;

    let width_offset = (scaled_width.saturating_sub(base_popup_area.width)) / 2;
    let height_offset = (scaled_height.saturating_sub(base_popup_area.height)) / 2;

    let popup_area = Rect {
        x: base_popup_area.x.saturating_sub(width_offset),
        y: base_popup_area.y.saturating_sub(height_offset),
        width: scaled_width.min(area.width),
        height: scaled_height.min(area.height),
    };

    loading_paragraph.render(popup_area, buf);

    // Add animated corners/decorations around the popup
    render_loading_corners(popup_area, buf, model.loading_counter);
}

/// Render executing screen
fn render_executing(model: &Model, area: Rect, buf: &mut Buffer) {
    let command = model.get_selected_command();
    let executing_text = format!(
        "Executing command...\n\nCommand: {:?}\n\nPress Esc to return to menu",
        command
    );

    let executing_block = Block::bordered()
        .title("⚡ Executing Command")
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Rounded)
        .style(Style::default().fg(Color::Yellow));

    let executing_paragraph = Paragraph::new(executing_text)
        .block(executing_block)
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Center);

    let popup_area = centered_rect(70, 50, area);
    executing_paragraph.render(popup_area, buf);
}

/// Render main menu with header and footer
fn render_menu(model: &mut Model, area: Rect, buf: &mut Buffer) {
    // Create layout with proper constraints
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(0)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Min(0),    // Menu content
            Constraint::Length(3), // Footer
        ])
        .split(area);

    // Get slide animation progress
    let slide_progress = model.effects.get_slide_in_progress();

    // Calculate slide offset (slide in from right)
    let slide_offset = ((1.0 - slide_progress) * area.width as f32 * 0.3) as u16;

    // Render header with slide animation
    let header_area = if slide_offset > 0 {
        Rect {
            x: chunks[0].x + slide_offset,
            y: chunks[0].y,
            width: chunks[0].width.saturating_sub(slide_offset),
            height: chunks[0].height,
        }
    } else {
        chunks[0]
    };

    let header_block = Block::bordered()
        .title("🤖 DroidTUI - Android Development Toolkit")
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Rounded)
        .style(Style::default().fg(Color::Green));

    let header = Paragraph::new("ADB Command Interface")
        .block(header_block)
        .style(Style::default().fg(Color::LightGreen))
        .alignment(Alignment::Center);

    if header_area.width > 0 {
        header.render(header_area, buf);
    }

    // Render menu with slide animation
    let menu_area = if slide_offset > 0 {
        Rect {
            x: chunks[1].x + slide_offset,
            y: chunks[1].y,
            width: chunks[1].width.saturating_sub(slide_offset),
            height: chunks[1].height,
        }
    } else {
        chunks[1]
    };

    // Wrap menu in a bordered block to ensure alignment
    let menu_block = Block::bordered()
        .title("📱 ADB Command Interface")
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Rounded)
        .style(Style::default().fg(Color::Green));

    let menu_inner = menu_block.inner(menu_area);

    if menu_area.width > 0 {
        menu_block.render(menu_area, buf);
        // Render menu content inside the bordered block
        (&model.menu).render(menu_inner, buf);
    }

    // Render footer with help
    let footer_block = Block::bordered()
        .title("Help")
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Rounded)
        .style(Style::default().fg(Color::Yellow));

    let footer_text = if model.menu.is_in_child_mode() {
        "↑/↓ or j/k: Navigate | Enter: Execute | ← Left/Backspace: Back | q/Esc: Quit"
    } else {
        "↑/↓ or j/k: Navigate | Enter/→: Options | q/Esc: Quit"
    };

    // Apply fade-in effect if entering child mode
    if model.menu.is_in_child_mode() && !model.effects.is_fade_in_complete() {
        let fade_progress = model.effects.get_fade_in_progress();

        // Apply fade effect to the entire menu area
        for y in area.top()..area.bottom() {
            for x in area.left()..area.right() {
                if let Some(cell) = buf.cell_mut((x, y)) {
                    // Simple fade effect - adjust alpha by modifying colors
                    match cell.fg {
                        Color::Green => {
                            let intensity = (255.0 * fade_progress) as u8;
                            cell.set_fg(Color::Rgb(0, intensity, 0));
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

    // Render footer with slide animation
    let footer_area = if slide_offset > 0 {
        Rect {
            x: chunks[2].x + slide_offset,
            y: chunks[2].y,
            width: chunks[2].width.saturating_sub(slide_offset),
            height: chunks[2].height,
        }
    } else {
        chunks[2]
    };

    let footer = Paragraph::new(footer_text)
        .block(footer_block)
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Center);

    if footer_area.width > 0 {
        footer.render(footer_area, buf);
    }
}

/// Render command result with scrolling support
fn render_result(model: &mut Model, area: Rect, buf: &mut Buffer) {
    let (title, color) = if model.command_result.is_some() {
        ("✅ Command Result", Color::Green)
    } else if model.command_error.is_some() {
        ("❌ Command Error", Color::Red)
    } else {
        ("📋 No Output", Color::Yellow)
    };

    // Get slide animation progress for result popup
    let slide_progress = model.effects.get_slide_in_progress();

    // Use larger area for results to accommodate more text
    let base_popup_area = centered_rect(80, 70, area);

    // Calculate slide offset (slide up from bottom)
    let slide_offset = ((1.0 - slide_progress) * base_popup_area.height as f32 * 0.5) as u16;

    let popup_area = if slide_offset > 0 {
        Rect {
            x: base_popup_area.x,
            y: base_popup_area.y + slide_offset,
            width: base_popup_area.width,
            height: base_popup_area.height.saturating_sub(slide_offset),
        }
    } else {
        base_popup_area
    };

    // Don't render if area is too small
    if popup_area.height < 5 {
        return;
    }

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
    let max_width = content_area.width.saturating_sub(4) as usize;
    model.update_wrapped_lines(max_width);

    // Determine visible content based on scroll position
    let total_lines = model.wrapped_lines.len();
    let start_line = model.scroll_position;
    let end_line = (start_line + visible_lines).min(total_lines);

    let visible_content: Vec<String> = model.wrapped_lines[start_line..end_line].to_vec();

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
        render_scrollbar(
            scrollbar_area,
            buf,
            total_lines,
            visible_lines,
            start_line,
            color,
        );
    }
}

/// Render a scrollbar indicator
fn render_scrollbar(
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
    let thumb_size =
        ((visible_lines as f64 / total_lines as f64) * scrollbar_height as f64).max(1.0) as usize;
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

/// Helper function to create a centered rect
/// Render animated corners around loading popup
fn render_loading_corners(area: Rect, buf: &mut Buffer, tick_count: u64) {
    if area.width < 4 || area.height < 4 {
        return;
    }

    // Animate corner characters
    let corner_chars = ['◢', '◣', '◤', '◥'];
    let corner_index = ((tick_count / 5) % 4) as usize;
    let corner_char = corner_chars[corner_index];

    // Pulsing color for corners
    let pulse = ((tick_count as f32 * 0.1).sin() * 0.5 + 0.5 * 255.0) as u8;
    let corner_color = Color::Rgb(pulse, pulse, 0);

    // Top-left corner
    if let Some(cell) = buf.cell_mut((area.x, area.y)) {
        cell.set_char(corner_char);
        cell.set_fg(corner_color);
    }

    // Top-right corner
    if let Some(cell) = buf.cell_mut((area.right().saturating_sub(1), area.y)) {
        cell.set_char(corner_char);
        cell.set_fg(corner_color);
    }

    // Bottom-left corner
    if let Some(cell) = buf.cell_mut((area.x, area.bottom().saturating_sub(1))) {
        cell.set_char(corner_char);
        cell.set_fg(corner_color);
    }

    // Bottom-right corner
    if let Some(cell) = buf.cell_mut((
        area.right().saturating_sub(1),
        area.bottom().saturating_sub(1),
    )) {
        cell.set_char(corner_char);
        cell.set_fg(corner_color);
    }
}

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
