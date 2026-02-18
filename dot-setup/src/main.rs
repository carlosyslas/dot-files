use color_eyre::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame, Terminal,
};
use std::io;
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

#[derive(PartialEq, Clone)]
enum AppState {
    Menu,
    GettingPassword,
    Installing,
    Stowing,
    Done,
}

struct App {
    state: AppState,
    selected_index: usize,
    menu_items: Vec<String>,
    output: Vec<String>,
    scroll: u16,
    sudo_password: String,
    running: AtomicBool,
}

impl App {
    fn new() -> Self {
        Self {
            state: AppState::Menu,
            selected_index: 0,
            menu_items: vec![
                "Install System".to_string(),
                "Stow Dotfiles".to_string(),
                "View Logs".to_string(),
                "Exit".to_string(),
            ],
            output: vec!["Welcome to dot-setup TUI".to_string()],
            scroll: 0,
            sudo_password: String::new(),
            running: AtomicBool::new(true),
        }
    }

    fn get_password(&mut self) {
        self.state = AppState::GettingPassword;
        self.output.clear();
        self.output.push("Enter sudo password:".to_string());
    }

    fn run_install(&mut self) {
        if self.sudo_password.is_empty() {
            self.get_password();
            return;
        }

        self.state = AppState::Installing;
        self.output.clear();
        self.output.push("Starting installation...".to_string());

        let password = self.sudo_password.clone();
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let commands: Vec<(&str, String)> = vec![
                ("Adding RPM Fusion repositories", format!("echo '{}' | sudo -S dnf install -y https://download1.rpmfusion.org/free/fedora/rpmfusion-free-release-$(rpm -E %fedora).noarch.rpm https://download1.rpmfusion.org/nonfree/fedora/rpmfusion-nonfree-release-$(rpm -E %fedora).noarch.rpm", password)),
                ("Adding Docker repository", format!("echo '{}' | sudo -S dnf config-manager addrepo --overwrite --from-repofile https://download.docker.com/linux/fedora/docker-ce.repo", password)),
                ("Adding Terra repository", format!("echo '{}' | sudo -S bash -c 'if ! rpm -q terra-release &>/dev/null; then dnf install --nogpgcheck --repofrompath terra,https://repos.fyralabs.com/terra$releasever terra-release; fi'", password)),
                ("Updating system", format!("echo '{}' | sudo -S dnf update -y", password)),
                ("Installing core packages", format!("echo '{}' | sudo -S dnf install -y vim stow alacritty git timeshift emacs yt-dlp imv mpv vlc zsh fastfetch bat ranger cargo jq yq fzf ripgrep fd-find eza bottom starship gh curl wget unzip tar gzip fuse polkit gnome-keyring seahorse libsodium pkgconfig", password)),
                ("Installing Docker", format!("echo '{}' | sudo -S dnf install -y docker-ce docker-ce-cli containerd.io docker-buildx-plugin docker-compose-plugin && echo '{}' | sudo -S systemctl enable --now docker", password, password)),
                ("Installing Flatpak apps", "flatpak remote-add --if-not-exists flathub https://dl.flathub.org/repo/flathub.flatpakrepo && flatpak install -y com.brave.Browser app.zen_browser.zen io.gitlab.theevilskeleton.Upscaler org.upscayl.Upscaler io.github.kolunmi.Bazaar org.qbittorrent.qBittorrent com.rafaelmardojai.Blanket com.github.johnfactotum.Foliate org.telegram.desktop".to_string()),
                ("Installing Homebrew", "/bin/bash -c \"$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)\"".to_string()),
                ("Installing Homebrew packages", "eval \"$(/home/linuxbrew/.linuxbrew/bin/brew shellenv)\" && brew install anomalyco/tap/opencode starship hugo tlrc uv".to_string()),
                ("Installing OpenCode Desktop", format!("echo '{}' | sudo -S dnf install -y https://opencode.ai/download/linux-x64-rpm", password)),
                ("Installing Cargo packages", "cargo install fnm bottom".to_string()),
                ("Installing Terra extras", format!("echo '{}' | sudo -S dnf install -y terra-release-extras", password)),
            ];

            for (desc, cmd) in commands {
                let _ = tx.send(format!("\n=== {} ===", desc));

                let output = Command::new("sh")
                    .args(["-c", &cmd])
                    .stdin(Stdio::piped())
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .output();

                match output {
                    Ok(out) => {
                        if !out.stdout.is_empty() {
                            let lines: Vec<String> = String::from_utf8_lossy(&out.stdout)
                                .lines()
                                .filter(|l| !l.contains("Password:"))
                                .map(|s| s.to_string())
                                .collect();
                            for line in lines.iter().take(20) {
                                let _ = tx.send(line.clone());
                            }
                        }
                        if !out.stderr.is_empty() {
                            let stderr = String::from_utf8_lossy(&out.stderr);
                            if !stderr.contains("Password:") && !stderr.is_empty() {
                                let _ = tx.send(format!("stderr: {}", stderr));
                            }
                        }
                        if out.status.success() {
                            let _ = tx.send(format!("✓ {} completed", desc));
                        } else {
                            let _ = tx.send(format!("✗ {} failed (exit code: {:?})", desc, out.status.code()));
                        }
                    }
                    Err(e) => {
                        let _ = tx.send(format!("Error: {}", e));
                    }
                }
            }

            let _ = tx.send("\n=== Setup complete! ===".to_string());
            let _ = tx.send("Consider restarting or running: exec zsh".to_string());
            let _ = tx.send("Press ESC to return to menu".to_string());
        });

        while self.running.load(Ordering::Relaxed) {
            match rx.try_recv() {
                Ok(msg) => {
                    self.output.push(msg);
                    if self.output.len() > 1000 {
                        self.output.remove(0);
                    }
                    self.scroll = (self.output.len() as u16).saturating_sub(1);
                }
                Err(mpsc::TryRecvError::Empty) => {
                    if !self.state.eq(&AppState::Installing) {
                        break;
                    }
                }
                Err(mpsc::TryRecvError::Disconnected) => {
                    break;
                }
            }
            thread::sleep(Duration::from_millis(50));
        }
    }

    fn run_stow(&mut self) {
        self.state = AppState::Stowing;
        self.output.clear();
        self.output.push("Starting stow...".to_string());

        let output = Command::new("sh")
            .args(["-c", "mkdir -p $HOME/.local/bin && stow -R -t $HOME/ --dotfiles ."])
            .output();

        match output {
            Ok(out) => {
                if !out.stdout.is_empty() {
                    self.output.push(String::from_utf8_lossy(&out.stdout).to_string());
                }
                if !out.stderr.is_empty() {
                    self.output.push(String::from_utf8_lossy(&out.stderr).to_string());
                }
                if out.status.success() {
                    self.output.push("✓ Stow completed successfully".to_string());
                } else {
                    self.output.push("✗ Stow failed".to_string());
                }
            }
            Err(e) => {
                self.output.push(format!("Error: {}", e));
            }
        }

        self.output.push("Press ESC to return to menu".to_string());
        self.scroll = (self.output.len() as u16).saturating_sub(1);
    }
}

fn main() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        eprintln!("Error: {:?}", err);
    }

    Ok(())
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, app: &mut App) -> Result<()> {
    let mut password_input = String::new();

    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match app.state {
                    AppState::GettingPassword => {
                        match key.code {
                            KeyCode::Enter => {
                                app.sudo_password = password_input.clone();
                                password_input.clear();
                                app.running.store(true, Ordering::Relaxed);
                                app.run_install();
                            }
                            KeyCode::Backspace => {
                                password_input.pop();
                            }
                            KeyCode::Esc => {
                                app.state = AppState::Menu;
                                password_input.clear();
                            }
                            KeyCode::Char(c) => {
                                password_input.push(c);
                            }
                            _ => {}
                        }
                    }
                    AppState::Menu => {
                        match key.code {
                            KeyCode::Up => {
                                if app.selected_index > 0 {
                                    app.selected_index -= 1;
                                }
                            }
                            KeyCode::Down => {
                                if app.selected_index < app.menu_items.len() - 1 {
                                    app.selected_index += 1;
                                }
                            }
                            KeyCode::Enter => {
                                match app.selected_index {
                                    0 => {
                                        app.running.store(true, Ordering::Relaxed);
                                        app.run_install();
                                    }
                                    1 => {
                                        app.run_stow();
                                    }
                                    2 => {
                                        app.state = AppState::Done;
                                    }
                                    3 => {
                                        app.running.store(false, Ordering::Relaxed);
                                        return Ok(());
                                    }
                                    _ => {}
                                }
                            }
                            KeyCode::Esc => {
                                app.running.store(false, Ordering::Relaxed);
                                return Ok(());
                            }
                            _ => {}
                        }
                    }
                    AppState::Installing | AppState::Stowing => {
                        if key.code == KeyCode::Esc {
                            app.state = AppState::Menu;
                            app.running.store(false, Ordering::Relaxed);
                            app.output.clear();
                            app.output.push("Welcome to dot-setup TUI".to_string());
                        }
                    }
                    AppState::Done => {
                        if key.code == KeyCode::Esc {
                            app.state = AppState::Menu;
                            app.output.clear();
                            app.output.push("Welcome to dot-setup TUI".to_string());
                        }
                    }
                }
            }
        }
    }
}

fn ui(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(frame.area());

    let title = Paragraph::new("dot-setup")
        .style(Style::default().fg(Color::Cyan))
        .block(Block::default().borders(Borders::ALL).title("Main Menu"));
    frame.render_widget(title, chunks[0]);

    match app.state {
        AppState::GettingPassword => {
            let prompt = Paragraph::new("Enter sudo password:")
                .style(Style::default().fg(Color::Yellow))
                .block(Block::default().borders(Borders::ALL).title("Password"));
            frame.render_widget(prompt, chunks[1]);
        }
        AppState::Menu => {
            let items: Vec<ListItem> = app
                .menu_items
                .iter()
                .enumerate()
                .map(|(i, item)| {
                    let style = if i == app.selected_index {
                        Style::default().fg(Color::Yellow).add_modifier(ratatui::style::Modifier::BOLD)
                    } else {
                        Style::default().fg(Color::White)
                    };
                    ListItem::new(Span::styled(item, style))
                })
                .collect();

            let list = List::new(items)
                .block(Block::default().borders(Borders::ALL).title("Select Option"))
                .style(Style::default().fg(Color::White));

            frame.render_widget(list, chunks[1]);
        }
        AppState::Installing => {
            let status = Line::from(vec![
                Span::raw("Installing "),
                Span::styled("...", Style::default().fg(Color::Yellow)),
            ]);
            let status_line = Paragraph::new(status)
                .block(Block::default().borders(Borders::ALL).title("Status"));
            frame.render_widget(status_line, chunks[1]);

            let output_text: Vec<Line> = app
                .output
                .iter()
                .map(|s| Line::from(s.as_str()))
                .collect();
            let output = Paragraph::new(output_text)
                .block(Block::default().borders(Borders::ALL).title("Installation Log"))
                .scroll((app.scroll, 0));
            frame.render_widget(output, chunks[1]);
        }
        AppState::Stowing => {
            let status = Line::from(vec![
                Span::raw("Stowing "),
                Span::styled("...", Style::default().fg(Color::Yellow)),
            ]);
            let status_line = Paragraph::new(status)
                .block(Block::default().borders(Borders::ALL).title("Status"));
            frame.render_widget(status_line, chunks[1]);

            let output_text: Vec<Line> = app
                .output
                .iter()
                .map(|s| Line::from(s.as_str()))
                .collect();
            let output = Paragraph::new(output_text)
                .block(Block::default().borders(Borders::ALL).title("Stow Log"))
                .scroll((app.scroll, 0));
            frame.render_widget(output, chunks[1]);
        }
        AppState::Done => {
            let output_text: Vec<Line> = app
                .output
                .iter()
                .map(|s| Line::from(s.as_str()))
                .collect();
            let output = Paragraph::new(output_text)
                .block(Block::default().borders(Borders::ALL).title("Logs"))
                .scroll((app.scroll, 0));
            frame.render_widget(output, chunks[1]);
        }
    }

    let help_text = match app.state {
        AppState::GettingPassword => "Type password | Enter Submit | Esc Cancel",
        _ => "↑↓ Navigate | Enter Select | Esc Back",
    };
    let help = Paragraph::new(help_text)
        .style(Style::default().fg(Color::DarkGray))
        .block(Block::default().borders(Borders::ALL));
    frame.render_widget(help, chunks[2]);
}
