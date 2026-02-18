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
use serde::Deserialize;
use std::fs;
use std::io;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[derive(PartialEq, Clone)]
enum AppState {
    Menu = 0,
    GettingPassword = 1,
    Installing = 2,
    Stowing = 3,
    Done = 4,
}

impl AppState {
    fn as_usize(&self) -> usize {
        match self {
            AppState::Menu => 0,
            AppState::GettingPassword => 1,
            AppState::Installing => 2,
            AppState::Stowing => 3,
            AppState::Done => 4,
        }
    }
    
    fn from_usize(val: usize) -> Self {
        match val {
            0 => AppState::Menu,
            1 => AppState::GettingPassword,
            2 => AppState::Installing,
            3 => AppState::Stowing,
            _ => AppState::Done,
        }
    }
}

#[derive(Deserialize, Clone)]
struct Config {
    repositories: Repositories,
    packages: Packages,
    commands: Commands,
}

#[derive(Deserialize, Clone)]
struct Repositories {
    #[serde(rename = "rpm_fusion_free")]
    rpm_fusion_free: String,
    #[serde(rename = "rpm_fusion_nonfree")]
    rpm_fusion_nonfree: String,
    docker: String,
    terra: String,
}

#[derive(Deserialize, Clone)]
struct Packages {
    #[serde(rename = "dnf")]
    dnf: PackageGroup,
    docker: PackageGroup,
    flatpak: FlatpakGroup,
    terra: PackageGroup,
    homebrew: HomebrewGroup,
    cargo: PackageGroup,
    opencode: OpenCodeGroup,
}

#[derive(Deserialize, Clone)]
struct PackageGroup {
    #[allow(dead_code)]
    description: String,
    packages: Vec<String>,
    #[serde(default)]
    enable_service: bool,
}

#[derive(Deserialize, Clone)]
struct FlatpakGroup {
    #[allow(dead_code)]
    description: String,
    remote: String,
    apps: Vec<String>,
}

#[derive(Deserialize, Clone)]
struct HomebrewGroup {
    #[allow(dead_code)]
    description: String,
    #[serde(rename = "install_script")]
    install_script: String,
    packages: Vec<String>,
}

#[derive(Deserialize, Clone)]
struct OpenCodeGroup {
    #[allow(dead_code)]
    description: String,
    url: String,
}

#[derive(Deserialize, Clone)]
struct Commands {
    update: String,
    #[serde(rename = "shell_init")]
    shell_init: String,
}

struct App {
    state: Arc<AtomicUsize>,
    selected_index: usize,
    menu_items: Vec<String>,
    output: Arc<Mutex<Vec<String>>>,
    scroll: u16,
    sudo_password: String,
    running: Arc<AtomicBool>,
    config: Config,
}

impl App {
    fn new() -> Self {
        let config = load_config().expect("Failed to load config");
        
        Self {
            state: Arc::new(AtomicUsize::new(0)),
            selected_index: 0,
            menu_items: vec![
                "Install System".to_string(),
                "Stow Dotfiles".to_string(),
                "View Logs".to_string(),
                "Exit".to_string(),
            ],
            output: Arc::new(Mutex::new(vec!["Welcome to dot-setup TUI".to_string()])),
            scroll: 0,
            sudo_password: String::new(),
            running: Arc::new(AtomicBool::new(true)),
            config,
        }
    }

    fn get_state(&self) -> AppState {
        AppState::from_usize(self.state.load(Ordering::Relaxed))
    }

    fn set_state(&self, new_state: AppState) {
        self.state.store(new_state.as_usize(), Ordering::Relaxed);
    }

    fn get_password(&mut self) {
        self.set_state(AppState::GettingPassword);
        let mut out = self.output.lock().unwrap();
        out.clear();
        out.push("Enter sudo password:".to_string());
    }

    fn run_install(&mut self) {
        if self.sudo_password.is_empty() {
            self.get_password();
            return;
        }

        self.set_state(AppState::Installing);
        {
            let mut out = self.output.lock().unwrap();
            out.clear();
            out.push("Starting installation...".to_string());
        }

        let password = self.sudo_password.clone();
        let config = self.config.clone();
        let output = Arc::clone(&self.output);
        let running = Arc::clone(&self.running);
        let state = Arc::clone(&self.state);

        thread::spawn(move || {
            let repo = &config.repositories;
            let pkg = &config.packages;
            let cmd = &config.commands;

            let commands: Vec<(&str, String)> = vec![
                ("Adding RPM Fusion repositories", format!(
                    "echo '{}' | sudo -S dnf install -y {} {}",
                    password, repo.rpm_fusion_free, repo.rpm_fusion_nonfree
                )),
                ("Adding Docker repository", format!(
                    "echo '{}' | sudo -S dnf config-manager addrepo --overwrite --from-repofile {}",
                    password, repo.docker
                )),
                ("Adding Terra repository", format!(
                    "echo '{}' | sudo -S bash -c 'if ! rpm -q terra-release &>/dev/null; then dnf install --nogpgcheck --repofrompath terra,{} terra-release; fi'",
                    password, repo.terra
                )),
                ("Updating system", format!("echo '{}' | sudo -S {}", password, cmd.update)),
                ("Installing core packages", format!(
                    "echo '{}' | sudo -S dnf install -y {}",
                    password, pkg.dnf.packages.join(" ")
                )),
                ("Installing Docker", format!(
                    "echo '{}' | sudo -S dnf install -y {}",
                    password, pkg.docker.packages.join(" ")
                )),
                ("Enabling Docker service", if pkg.docker.enable_service {
                    format!("echo '{}' | sudo -S systemctl enable --now docker", password)
                } else {
                    "".to_string()
                }),
                ("Adding Flatpak remote", format!(
                    "flatpak remote-add --if-not-exists {} https://dl.flathub.org/repo/flathub.flatpakrepo",
                    pkg.flatpak.remote
                )),
                ("Installing Flatpak apps", format!(
                    "flatpak install -y {}",
                    pkg.flatpak.apps.join(" ")
                )),
                ("Installing Homebrew", format!(
                    "/bin/bash -c \"$(curl -fsSL {})\"",
                    pkg.homebrew.install_script
                )),
                ("Installing Homebrew packages", format!(
                    "eval \"$( {})\" && brew install {}",
                    cmd.shell_init,
                    pkg.homebrew.packages.join(" ")
                )),
                ("Installing OpenCode Desktop", format!(
                    "echo '{}' | sudo -S dnf install -y {}",
                    password, pkg.opencode.url
                )),
                ("Installing Cargo packages", format!(
                    "cargo install {}",
                    pkg.cargo.packages.join(" ")
                )),
                ("Installing Terra extras", format!(
                    "echo '{}' | sudo -S dnf install -y {}",
                    password, pkg.terra.packages.join(" ")
                )),
            ];

            for (desc, cmd_str) in commands {
                if !running.load(Ordering::Relaxed) {
                    let mut out = output.lock().unwrap();
                    out.push("\n=== Installation cancelled ===".to_string());
                    break;
                }
                
                if cmd_str.is_empty() {
                    continue;
                }
                
                {
                    let mut out = output.lock().unwrap();
                    out.push(format!("\n=== {} ===", desc));
                }

                let output_child = Command::new("sh")
                    .args(["-c", &cmd_str])
                    .stdin(Stdio::piped())
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .output();

                match output_child {
                    Ok(cmd_out) => {
                        if !cmd_out.stdout.is_empty() {
                            let lines: Vec<String> = String::from_utf8_lossy(&cmd_out.stdout)
                                .lines()
                                .filter(|l| !l.contains("Password:"))
                                .map(|s| s.to_string())
                                .collect();
                            let mut out = output.lock().unwrap();
                            for line in lines.iter().take(20) {
                                out.push(line.clone());
                            }
                        }
                        if !cmd_out.stderr.is_empty() {
                            let stderr = String::from_utf8_lossy(&cmd_out.stderr);
                            if !stderr.contains("Password:") && !stderr.is_empty() {
                                let mut out = output.lock().unwrap();
                                out.push(format!("stderr: {}", stderr));
                            }
                        }
                        let mut out = output.lock().unwrap();
                        if out.len() > 1000 {
                            out.drain(0..500);
                        }
                        if cmd_out.status.success() {
                            out.push(format!("✓ {} completed", desc));
                        } else {
                            out.push(format!("✗ {} failed (exit code: {:?})", desc, cmd_out.status.code()));
                        }
                    }
                    Err(e) => {
                        let mut out = output.lock().unwrap();
                        out.push(format!("Error: {}", e));
                    }
                }
            }

            if running.load(Ordering::Relaxed) {
                let mut out = output.lock().unwrap();
                out.push("\n=== Setup complete! ===".to_string());
                out.push("Consider restarting or running: exec zsh".to_string());
                out.push("Press ESC to return to menu".to_string());
            }
            
            state.store(AppState::Done.as_usize(), Ordering::Relaxed);
        });
    }

    fn run_stow(&mut self) {
        self.set_state(AppState::Stowing);
        {
            let mut out = self.output.lock().unwrap();
            out.clear();
            out.push("Starting stow...".to_string());
        }

        let cmd_output = Command::new("sh")
            .args(["-c", "mkdir -p $HOME/.local/bin && stow -R -t $HOME/ --dotfiles ."])
            .output();

        match cmd_output {
            Ok(co) => {
                if !co.stdout.is_empty() {
                    let mut out = self.output.lock().unwrap();
                    out.push(String::from_utf8_lossy(&co.stdout).to_string());
                }
                if !co.stderr.is_empty() {
                    let mut out = self.output.lock().unwrap();
                    out.push(String::from_utf8_lossy(&co.stderr).to_string());
                }
                let mut out = self.output.lock().unwrap();
                if co.status.success() {
                    out.push("✓ Stow completed successfully".to_string());
                } else {
                    out.push("✗ Stow failed".to_string());
                }
            }
            Err(e) => {
                let mut out = self.output.lock().unwrap();
                out.push(format!("Error: {}", e));
            }
        }

        {
            let mut out = self.output.lock().unwrap();
            out.push("Press ESC to return to menu".to_string());
        }
        self.set_state(AppState::Done);
    }
}

fn load_config() -> Result<Config> {
    let config_path = get_config_path()?;
    let content = fs::read_to_string(&config_path)?;
    let config: Config = toml::from_str(&content)?;
    Ok(config)
}

fn get_config_path() -> Result<PathBuf> {
    let exe_path = std::env::current_exe()?;
    let config_path = exe_path.with_file_name("config.toml");
    
    if config_path.exists() {
        return Ok(config_path);
    }
    
    let fallback = PathBuf::from("config.toml");
    if fallback.exists() {
        return Ok(fallback);
    }
    
    Ok(config_path)
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
        let state = app.get_state();
        
        terminal.draw(|f| ui(f, app))?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match state {
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
                                    app.set_state(AppState::Menu);
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
                                            app.set_state(AppState::Done);
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
                                app.running.store(false, Ordering::Relaxed);
                                app.set_state(AppState::Menu);
                                let mut out = app.output.lock().unwrap();
                                out.clear();
                                out.push("Welcome to dot-setup TUI".to_string());
                            }
                        }
                        AppState::Done => {
                            if key.code == KeyCode::Esc {
                                app.set_state(AppState::Menu);
                                let mut out = app.output.lock().unwrap();
                                out.clear();
                                out.push("Welcome to dot-setup TUI".to_string());
                            }
                        }
                    }
                }
            }
        }
        
        let current_state = app.get_state();
        if current_state == AppState::Installing || current_state == AppState::Stowing {
            let out = app.output.lock().unwrap();
            app.scroll = (out.len() as u16).saturating_sub(1);
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

    let state = app.get_state();
    let output = app.output.lock().unwrap();

    match state {
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

            let output_text: Vec<Line> = output
                .iter()
                .map(|s| Line::from(s.as_str()))
                .collect();
            let output_widget = Paragraph::new(output_text)
                .block(Block::default().borders(Borders::ALL).title("Installation Log"))
                .scroll((app.scroll, 0));
            frame.render_widget(output_widget, chunks[1]);
        }
        AppState::Stowing => {
            let status = Line::from(vec![
                Span::raw("Stowing "),
                Span::styled("...", Style::default().fg(Color::Yellow)),
            ]);
            let status_line = Paragraph::new(status)
                .block(Block::default().borders(Borders::ALL).title("Status"));
            frame.render_widget(status_line, chunks[1]);

            let output_text: Vec<Line> = output
                .iter()
                .map(|s| Line::from(s.as_str()))
                .collect();
            let output_widget = Paragraph::new(output_text)
                .block(Block::default().borders(Borders::ALL).title("Stow Log"))
                .scroll((app.scroll, 0));
            frame.render_widget(output_widget, chunks[1]);
        }
        AppState::Done => {
            let output_text: Vec<Line> = output
                .iter()
                .map(|s| Line::from(s.as_str()))
                .collect();
            let output_widget = Paragraph::new(output_text)
                .block(Block::default().borders(Borders::ALL).title("Logs"))
                .scroll((app.scroll, 0));
            frame.render_widget(output_widget, chunks[1]);
        }
    }

    let help_text = match state {
        AppState::GettingPassword => "Type password | Enter Submit | Esc Cancel",
        _ => "↑↓ Navigate | Enter Select | Esc Back",
    };
    let help = Paragraph::new(help_text)
        .style(Style::default().fg(Color::DarkGray))
        .block(Block::default().borders(Borders::ALL));
    frame.render_widget(help, chunks[2]);
}
