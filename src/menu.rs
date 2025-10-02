// Removed unused import: get_menu_border_color
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, List, ListItem, ListState, Paragraph, Widget},
};

#[derive(Debug, Clone)]
pub struct MenuItem {
    pub label: String,
    pub description: String,
    pub command: String,
    pub children: Vec<MenuChild>,
}

#[derive(Debug, Clone)]
pub struct MenuChild {
    pub label: String,
    pub description: String,
    pub command: String,
}

#[derive(Debug)]
pub struct Menu {
    pub items: Vec<MenuItem>,
    pub state: ListState,
    pub selected: usize,
    pub tick_count: u64,
    pub last_selected: usize,
    pub position_change_boost: u64,
    pub in_child_mode: bool,
    pub child_selected: usize,
    pub child_state: ListState,
}

impl Default for Menu {
    fn default() -> Self {
        let items = vec![
            MenuItem {
                label: "📱 List Devices".to_string(),
                description: "Show all connected Android devices with their status".to_string(),
                command: "adb devices -l".to_string(),
                children: vec![
                    MenuChild {
                        label: "📋 List Connected Devices".to_string(),
                        description: "Show basic device list".to_string(),
                        command: "adb devices".to_string(),
                    },
                    MenuChild {
                        label: "📝 Detailed Device Info".to_string(),
                        description: "Show devices with detailed information".to_string(),
                        command: "adb devices -l".to_string(),
                    },
                    MenuChild {
                        label: "🔍 Device Serial Numbers".to_string(),
                        description: "List only device serial numbers".to_string(),
                        command: "adb devices | grep -v 'List of devices' | awk '{print $1}'".to_string(),
                    },
                ],
            },
            MenuItem {
                label: "📋 List Packages".to_string(),
                description: "List all installed packages on the device".to_string(),
                command: "adb shell pm list packages -f".to_string(),
                children: vec![
                    MenuChild {
                        label: "📦 All Packages".to_string(),
                        description: "List all installed packages".to_string(),
                        command: "adb shell pm list packages".to_string(),
                    },
                    MenuChild {
                        label: "📁 Packages with Paths".to_string(),
                        description: "List packages with file paths".to_string(),
                        command: "adb shell pm list packages -f".to_string(),
                    },
                    MenuChild {
                        label: "👤 User Packages Only".to_string(),
                        description: "List only user-installed packages".to_string(),
                        command: "adb shell pm list packages -3".to_string(),
                    },
                    MenuChild {
                        label: "⚙️ System Packages Only".to_string(),
                        description: "List only system packages".to_string(),
                        command: "adb shell pm list packages -s".to_string(),
                    },
                ],
            },
            MenuItem {
                label: "🔋 Battery Info".to_string(),
                description: "Display detailed battery information".to_string(),
                command: "adb shell dumpsys battery".to_string(),
                children: vec![
                    MenuChild {
                        label: "🔋 Full Battery Status".to_string(),
                        description: "Complete battery information".to_string(),
                        command: "adb shell dumpsys battery".to_string(),
                    },
                    MenuChild {
                        label: "⚡ Battery Level".to_string(),
                        description: "Show just battery percentage".to_string(),
                        command: "adb shell dumpsys battery | grep level".to_string(),
                    },
                    MenuChild {
                        label: "🔌 Charging Status".to_string(),
                        description: "Show charging state".to_string(),
                        command: "adb shell dumpsys battery | grep 'AC powered\\|USB powered\\|status'".to_string(),
                    },
                ],
            },
            MenuItem {
                label: "💾 Memory Usage".to_string(),
                description: "Show memory usage statistics".to_string(),
                command: "adb shell dumpsys meminfo".to_string(),
                children: vec![
                    MenuChild {
                        label: "📊 System Memory".to_string(),
                        description: "Overall system memory usage".to_string(),
                        command: "adb shell dumpsys meminfo".to_string(),
                    },
                    MenuChild {
                        label: "📱 Available Memory".to_string(),
                        description: "Show available memory".to_string(),
                        command: "adb shell cat /proc/meminfo | grep -E 'MemTotal|MemFree|MemAvailable'".to_string(),
                    },
                    MenuChild {
                        label: "🔝 Top Memory Apps".to_string(),
                        description: "Apps using most memory".to_string(),
                        command: "adb shell 'dumpsys meminfo | grep -E \"(Total|TOTAL)\" | head -10'".to_string(),
                    },
                ],
            },
            MenuItem {
                label: "📊 CPU Info".to_string(),
                description: "Display CPU information and usage".to_string(),
                command: "adb shell cat /proc/cpuinfo".to_string(),
                children: vec![
                    MenuChild {
                        label: "🔧 CPU Details".to_string(),
                        description: "Detailed CPU information".to_string(),
                        command: "adb shell cat /proc/cpuinfo".to_string(),
                    },
                    MenuChild {
                        label: "⚡ CPU Usage".to_string(),
                        description: "Top processes by CPU usage".to_string(),
                        command: "adb shell 'ps -eo PID,PPID,USER,COMM | head -20'".to_string(),
                    },
                    MenuChild {
                        label: "📈 Load Average".to_string(),
                        description: "System load average".to_string(),
                        command: "adb shell cat /proc/loadavg".to_string(),
                    },
                ],
            },
            MenuItem {
                label: "🔗 Network Info".to_string(),
                description: "Show network connectivity information".to_string(),
                command: "adb shell dumpsys connectivity".to_string(),
                children: vec![
                    MenuChild {
                        label: "🌐 Connectivity Status".to_string(),
                        description: "Network connectivity details".to_string(),
                        command: "adb shell dumpsys connectivity".to_string(),
                    },
                    MenuChild {
                        label: "📶 WiFi Info".to_string(),
                        description: "WiFi connection details".to_string(),
                        command: "adb shell dumpsys wifi".to_string(),
                    },
                    MenuChild {
                        label: "🔗 IP Configuration".to_string(),
                        description: "Network interface configuration".to_string(),
                        command: "adb shell ip addr show".to_string(),
                    },
                ],
            },
            MenuItem {
                label: "📱 Device Properties".to_string(),
                description: "Get all device system properties".to_string(),
                command: "adb shell getprop".to_string(),
                children: vec![
                    MenuChild {
                        label: "📋 All Properties".to_string(),
                        description: "Show all system properties".to_string(),
                        command: "adb shell getprop".to_string(),
                    },
                    MenuChild {
                        label: "🏷️ Device Model".to_string(),
                        description: "Show device model and brand".to_string(),
                        command: "adb shell getprop | grep -E 'ro.product.model|ro.product.brand|ro.product.name'".to_string(),
                    },
                    MenuChild {
                        label: "🔢 Android Version".to_string(),
                        description: "Show Android version info".to_string(),
                        command: "adb shell getprop | grep -E 'ro.build.version|ro.build.id'".to_string(),
                    },
                ],
            },
            MenuItem {
                label: "🎯 Running Processes".to_string(),
                description: "List all running processes".to_string(),
                command: "adb shell ps -A".to_string(),
                children: vec![
                    MenuChild {
                        label: "📋 All Processes".to_string(),
                        description: "List all running processes".to_string(),
                        command: "adb shell ps -A".to_string(),
                    },
                    MenuChild {
                        label: "🔝 Top Processes".to_string(),
                        description: "Top 20 running processes".to_string(),
                        command: "adb shell 'ps -A | head -20'".to_string(),
                    },
                    MenuChild {
                        label: "👤 User Processes".to_string(),
                        description: "User application processes only".to_string(),
                        command: "adb shell \"ps -A | grep -v 'system' | grep -v 'root' | head -15\"".to_string(),
                    },
                    MenuChild {
                        label: "📊 Process Details".to_string(),
                        description: "Detailed process information with formatting".to_string(),
                        command: "adb shell \"echo 'PID    USER         COMMAND'; echo '---    ----         -------'; ps -A | awk '{printf \"%-6s %-12s %s\\n\", \\$2, \\$1, \\$9}' | head -20\"".to_string(),
                    },
                ],
            },
            MenuItem {
                label: "📊 System Services".to_string(),
                description: "List all system services status".to_string(),
                command: "adb shell service list".to_string(),
                children: vec![
                    MenuChild {
                        label: "📋 All Services".to_string(),
                        description: "List all system services".to_string(),
                        command: "adb shell service list".to_string(),
                    },
                    MenuChild {
                        label: "🔧 Running Services".to_string(),
                        description: "Show only running services".to_string(),
                        command: "adb shell dumpsys activity services".to_string(),
                    },
                    MenuChild {
                        label: "📱 App Services".to_string(),
                        description: "Application services only".to_string(),
                        command: "adb shell dumpsys activity services | grep -A 5 'ServiceRecord'".to_string(),
                    },
                ],
            },
            MenuItem {
                label: "📷 Screenshot".to_string(),
                description: "Take and save device screenshots".to_string(),
                command: "adb shell screencap -p /sdcard/screenshot.png && adb pull /sdcard/screenshot.png ./".to_string(),
                children: vec![
                    MenuChild {
                        label: "📸 Take Screenshot".to_string(),
                        description: "Take screenshot and save to current directory".to_string(),
                        command: "adb shell screencap -p /sdcard/screenshot.png && adb pull /sdcard/screenshot.png ./".to_string(),
                    },
                    MenuChild {
                        label: "📁 Screenshot to Device".to_string(),
                        description: "Take screenshot and keep on device".to_string(),
                        command: "adb shell screencap -p /sdcard/screenshot_$(date +%Y%m%d_%H%M%S).png".to_string(),
                    },
                    MenuChild {
                        label: "🖼️ View Screenshot Path".to_string(),
                        description: "Show where screenshots are saved".to_string(),
                        command: "adb shell ls -la /sdcard/screenshot*.png".to_string(),
                    },
                ],
            },
            MenuItem {
                label: "🔄 Reboot Device".to_string(),
                description: "Reboot the connected device".to_string(),
                command: "adb reboot".to_string(),
                children: vec![
                    MenuChild {
                        label: "🔄 Normal Reboot".to_string(),
                        description: "Reboot device normally".to_string(),
                        command: "adb reboot".to_string(),
                    },
                    MenuChild {
                        label: "⚡ Fast Reboot".to_string(),
                        description: "Fast reboot (bootloader)".to_string(),
                        command: "adb reboot bootloader".to_string(),
                    },
                    MenuChild {
                        label: "🔧 Recovery Mode".to_string(),
                        description: "Reboot to recovery mode".to_string(),
                        command: "adb reboot recovery".to_string(),
                    },
                ],
            },
            MenuItem {
                label: "📜 System Log".to_string(),
                description: "View recent system logs (last 100 lines)".to_string(),
                command: "adb logcat -d -t 100".to_string(),
                children: vec![
                    MenuChild {
                        label: "📜 Recent Logs".to_string(),
                        description: "Last 100 log entries".to_string(),
                        command: "adb logcat -d -t 100".to_string(),
                    },
                    MenuChild {
                        label: "🚨 Error Logs Only".to_string(),
                        description: "Show only error messages".to_string(),
                        command: "adb logcat -d *:E".to_string(),
                    },
                    MenuChild {
                        label: "⚠️ Warning and Error".to_string(),
                        description: "Show warnings and errors".to_string(),
                        command: "adb logcat -d *:W".to_string(),
                    },
                    MenuChild {
                        label: "🔄 Clear Logs".to_string(),
                        description: "Clear the log buffer".to_string(),
                        command: "adb logcat -c".to_string(),
                    },
                ],
            },
            MenuItem {
                label: "🔍 ADB Version".to_string(),
                description: "Display ADB version information".to_string(),
                command: "adb version".to_string(),
                children: vec![
                    MenuChild {
                        label: "🔍 ADB Version".to_string(),
                        description: "Show ADB version".to_string(),
                        command: "adb version".to_string(),
                    },
                    MenuChild {
                        label: "🔧 ADB Help".to_string(),
                        description: "Show ADB help information".to_string(),
                        command: "adb help".to_string(),
                    },
                    MenuChild {
                        label: "📍 ADB Path".to_string(),
                        description: "Show where ADB is installed".to_string(),
                        command: "which adb".to_string(),
                    },
                ],
            },
            MenuItem {
                label: "🧪 Test Scrolling".to_string(),
                description: "Test command with long output to demonstrate scrolling".to_string(),
                command: "./test_long_output.sh".to_string(),
                children: vec![
                    MenuChild {
                        label: "📜 Long Output Test".to_string(),
                        description: "Generate long output to test scrolling".to_string(),
                        command: "./test_long_output.sh".to_string(),
                    },
                    MenuChild {
                        label: "📋 List Directory".to_string(),
                        description: "List current directory contents".to_string(),
                        command: "ls -la".to_string(),
                    },
                    MenuChild {
                        label: "🔍 Show Environment".to_string(),
                        description: "Display environment variables".to_string(),
                        command: "env".to_string(),
                    },
                    MenuChild {
                        label: "📏 Wide Content Test".to_string(),
                        description: "Test wide content and text wrapping".to_string(),
                        command: "./test_wide_output.sh".to_string(),
                    },
                ],
            },
        ];

        let mut state = ListState::default();
        state.select(Some(0));

        let mut child_state = ListState::default();
        child_state.select(Some(0));

        Self {
            items,
            state,
            selected: 0,
            tick_count: 0,
            last_selected: 0,
            position_change_boost: 0,
            in_child_mode: false,
            child_selected: 0,
            child_state,
        }
    }
}

impl Menu {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn tick(&mut self) {
        self.tick_count += 1;

        // Reduce boost over time
        if self.position_change_boost > 0 {
            self.position_change_boost = self.position_change_boost.saturating_sub(1);
        }
    }

    pub fn next(&mut self) {
        if self.in_child_mode {
            let current_item = &self.items[self.selected];
            let i = if self.child_selected >= current_item.children.len() - 1 {
                0
            } else {
                self.child_selected + 1
            };
            self.child_selected = i;
            self.child_state.select(Some(i));
        } else {
            let i = match self.state.selected() {
                Some(i) => {
                    if i >= self.items.len() - 1 {
                        0
                    } else {
                        i + 1
                    }
                }
                None => 0,
            };
            self.state.select(Some(i));
            self.last_selected = self.selected;
            self.selected = i;

            // Add boost for position change
            if self.last_selected != self.selected {
                self.position_change_boost = 50; // Boost duration
            }
        }
    }

    pub fn previous(&mut self) {
        if self.in_child_mode {
            let current_item = &self.items[self.selected];
            let i = if self.child_selected == 0 {
                current_item.children.len() - 1
            } else {
                self.child_selected - 1
            };
            self.child_selected = i;
            self.child_state.select(Some(i));
        } else {
            let i = match self.state.selected() {
                Some(i) => {
                    if i == 0 {
                        self.items.len() - 1
                    } else {
                        i - 1
                    }
                }
                None => 0,
            };
            self.state.select(Some(i));
            self.last_selected = self.selected;
            self.selected = i;

            // Add boost for position change
            if self.last_selected != self.selected {
                self.position_change_boost = 50; // Boost duration
            }
        }
    }

    pub fn get_selected_item(&self) -> Option<&MenuItem> {
        self.items.get(self.selected)
    }

    pub fn get_selected_command(&self) -> Option<String> {
        if self.in_child_mode {
            if let Some(item) = self.items.get(self.selected) {
                item.children
                    .get(self.child_selected)
                    .map(|child| child.command.clone())
            } else {
                None
            }
        } else {
            self.items
                .get(self.selected)
                .map(|item| item.command.clone())
        }
    }

    pub fn enter_child_mode(&mut self) {
        if let Some(item) = self.items.get(self.selected) {
            if !item.children.is_empty() {
                self.in_child_mode = true;
                self.child_selected = 0;
                self.child_state.select(Some(0));
            }
        }
    }

    pub fn trigger_fade_in(&mut self) -> bool {
        // Return true if fade-in should be triggered
        self.in_child_mode
    }

    pub fn exit_child_mode(&mut self) {
        self.in_child_mode = false;
        self.child_selected = 0;
        self.child_state.select(Some(0));
    }

    pub fn is_in_child_mode(&self) -> bool {
        self.in_child_mode
    }
}

impl Widget for &Menu {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Split the area horizontally with a separator column
        let chunks = if self.in_child_mode {
            Layout::default()
                .direction(Direction::Horizontal)
                .margin(0)
                .constraints([
                    Constraint::Percentage(40),
                    Constraint::Length(1), // Separator
                    Constraint::Percentage(60),
                ])
                .split(area)
        } else {
            Layout::default()
                .direction(Direction::Horizontal)
                .margin(0)
                .constraints([
                    Constraint::Percentage(60),
                    Constraint::Length(1), // Separator
                    Constraint::Percentage(40),
                ])
                .split(area)
        };

        if self.in_child_mode {
            // Render parent menu (smaller)
            let parent_items: Vec<ListItem> = self
                .items
                .iter()
                .enumerate()
                .map(|(i, item)| {
                    let style = if i == self.selected {
                        Style::default()
                            .fg(Color::Yellow)
                            .add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(Color::DarkGray)
                    };
                    ListItem::new(Line::from(vec![Span::styled(item.label.clone(), style)]))
                })
                .collect();

            let parent_block = Block::default()
                .title("📂 Categories")
                .title_alignment(Alignment::Center)
                .style(Style::default().fg(Color::DarkGray));

            let parent_list = List::new(parent_items).block(parent_block);

            ratatui::widgets::StatefulWidget::render(
                parent_list,
                chunks[0],
                buf,
                &mut self.state.clone(),
            );

            // Render vertical separator
            for y in area.top()..area.bottom() {
                if let Some(cell) = buf.cell_mut((chunks[1].x, y)) {
                    cell.set_char('│');
                    cell.set_fg(Color::DarkGray);
                }
            }

            // Render child menu (larger)
            if let Some(current_item) = self.items.get(self.selected) {
                let child_items: Vec<ListItem> = current_item
                    .children
                    .iter()
                    .enumerate()
                    .map(|(i, child)| {
                        let style = if i == self.child_selected {
                            Style::default()
                                .fg(Color::Black)
                                .bg(Color::Green)
                                .add_modifier(Modifier::BOLD)
                        } else {
                            Style::default().fg(Color::White)
                        };
                        ListItem::new(Line::from(vec![Span::styled(child.label.clone(), style)]))
                    })
                    .collect();

                let child_border_color = Color::Green;

                let child_block = Block::default()
                    .title(format!("│ {} Options", current_item.label))
                    .title_alignment(Alignment::Left)
                    .style(Style::default().fg(child_border_color));

                let child_list = List::new(child_items).block(child_block).highlight_style(
                    Style::default()
                        .fg(Color::Black)
                        .bg(Color::Green)
                        .add_modifier(Modifier::BOLD),
                );

                ratatui::widgets::StatefulWidget::render(
                    child_list,
                    chunks[2],
                    buf,
                    &mut self.child_state.clone(),
                );
            }
        } else {
            // Render main menu list
            let items: Vec<ListItem> = self
                .items
                .iter()
                .enumerate()
                .map(|(i, item)| {
                    let style = if i == self.selected {
                        Style::default()
                            .fg(Color::Black)
                            .bg(Color::Green)
                            .add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(Color::White)
                    };

                    let label_with_indicator = if item.children.is_empty() {
                        item.label.clone()
                    } else {
                        format!("{} ▶", item.label)
                    };

                    ListItem::new(Line::from(vec![Span::styled(label_with_indicator, style)]))
                })
                .collect();

            let menu_border_color = Color::Green;

            let menu_block = Block::default()
                .title("📱 ADB Commands")
                .title_alignment(Alignment::Left)
                .style(Style::default().fg(menu_border_color));

            let menu_list = List::new(items).block(menu_block).highlight_style(
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            );

            ratatui::widgets::StatefulWidget::render(
                menu_list,
                chunks[0],
                buf,
                &mut self.state.clone(),
            );

            // Render vertical separator
            for y in area.top()..area.bottom() {
                if let Some(cell) = buf.cell_mut((chunks[1].x, y)) {
                    cell.set_char('│');
                    cell.set_fg(Color::DarkGray);
                }
            }

            // Render description panel with dynamic styling
            if let Some(selected_item) = self.get_selected_item() {
                let desc_color = Color::DarkGray;

                let description_block = Block::default()
                    .title("│ Description")
                    .title_alignment(Alignment::Left)
                    .style(Style::default().fg(desc_color));

                let help_text = if selected_item.children.is_empty() {
                    "💡 Use ↑/↓ to navigate | ⏎ Enter to execute | q/Esc to quit"
                } else {
                    "💡 Use ↑/↓ to navigate | ⏎ Enter for options | q/Esc to quit"
                };

                let description_text = format!(
                    "{}\n\n🔧 Command:\n{}\n\n{}",
                    selected_item.description, selected_item.command, help_text
                );

                let description = Paragraph::new(description_text)
                    .block(description_block)
                    .style(Style::default().fg(Color::White))
                    .alignment(Alignment::Left)
                    .wrap(ratatui::widgets::Wrap { trim: true });

                description.render(chunks[2], buf);
            }
        }
    }
}
