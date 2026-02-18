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

#[derive(PartialEq, Clone, Copy)]
enum AppState {
    Selection = 0,
    Confirm = 1,
    GettingPassword = 2,
    Running = 3,
    Done = 4,
}

impl AppState {
    fn as_usize(&self) -> usize {
        *self as usize
    }
    
    fn from_usize(val: usize) -> Self {
        match val {
            0 => AppState::Selection,
            1 => AppState::Confirm,
            2 => AppState::GettingPassword,
            3 => AppState::Running,
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

#[derive(Clone)]
struct Task {
    id: String,
    name: String,
    enabled: bool,
    is_install: bool,
}

#[derive(Clone, PartialEq)]
enum StepStatus {
    Pending,
    Running,
    Completed,
    Failed,
}

struct Step {
    name: String,
    status: StepStatus,
}

struct App {
    state: Arc<AtomicUsize>,
    selected_index: usize,
    tasks: Vec<Task>,
    output: Arc<Mutex<Vec<String>>>,
    steps: Arc<Mutex<Vec<Step>>>,
    current_step: Arc<AtomicUsize>,
    scroll: u16,
    sudo_password: String,
    running: Arc<AtomicBool>,
    config: Config,
    focus: Focus,
}

#[derive(PartialEq, Clone, Copy)]
enum Focus {
    TaskList,
    Buttons,
}

impl App {
    fn new() -> Self {
        let config = load_config().expect("Failed to load config");
        
        let tasks = vec![
            Task { id: "repos".to_string(), name: "Add Repositories (RPM Fusion, Docker, Terra)".to_string(), enabled: true, is_install: true },
            Task { id: "dnf".to_string(), name: "Install System Packages".to_string(), enabled: true, is_install: true },
            Task { id: "docker".to_string(), name: "Install Docker".to_string(), enabled: true, is_install: true },
            Task { id: "flatpak".to_string(), name: "Install Flatpak Apps".to_string(), enabled: true, is_install: true },
            Task { id: "homebrew".to_string(), name: "Install Homebrew".to_string(), enabled: true, is_install: true },
            Task { id: "opencode".to_string(), name: "Install OpenCode".to_string(), enabled: true, is_install: true },
            Task { id: "cargo".to_string(), name: "Install Cargo Packages".to_string(), enabled: true, is_install: true },
            Task { id: "terra".to_string(), name: "Install Terra Extras".to_string(), enabled: true, is_install: true },
            Task { id: "stow".to_string(), name: "Stow Dotfiles".to_string(), enabled: true, is_install: false },
        ];

        Self {
            state: Arc::new(AtomicUsize::new(0)),
            selected_index: 0,
            tasks,
            output: Arc::new(Mutex::new(vec!["Welcome to dot-setup".to_string()])),
            steps: Arc::new(Mutex::new(Vec::new())),
            current_step: Arc::new(AtomicUsize::new(0)),
            scroll: 0,
            sudo_password: String::new(),
            running: Arc::new(AtomicBool::new(true)),
            config,
            focus: Focus::TaskList,
        }
    }

    fn get_state(&self) -> AppState {
        AppState::from_usize(self.state.load(Ordering::Relaxed))
    }

    fn set_state(&self, new_state: AppState) {
        self.state.store(new_state.as_usize(), Ordering::Relaxed);
    }

    fn get_enabled_tasks(&self) -> Vec<Task> {
        self.tasks.iter().filter(|t| t.enabled).cloned().collect()
    }

    fn toggle_task(&mut self) {
        if self.selected_index < self.tasks.len() {
            self.tasks[self.selected_index].enabled = !self.tasks[self.selected_index].enabled;
        }
    }

    fn get_password(&mut self) {
        self.set_state(AppState::GettingPassword);
        let mut out = self.output.lock().unwrap();
        out.clear();
        out.push("Enter sudo password:".to_string());
    }

    fn start_tasks(&mut self) {
        let enabled_tasks = self.get_enabled_tasks();
        
        if enabled_tasks.is_empty() {
            return;
        }

        if enabled_tasks.iter().any(|t| t.is_install) && self.sudo_password.is_empty() {
            self.get_password();
            return;
        }

        self.set_state(AppState::Running);
        
        let mut steps = Vec::new();
        for task in &enabled_tasks {
            steps.push(Step { name: task.name.clone(), status: StepStatus::Pending });
        }
        {
            let mut s = self.steps.lock().unwrap();
            *s = steps;
        }
        self.current_step.store(0, Ordering::Relaxed);
        
        {
            let mut out = self.output.lock().unwrap();
            out.clear();
            out.push("Starting tasks...".to_string());
        }

        let password = self.sudo_password.clone();
        let config = self.config.clone();
        let tasks: Vec<Task> = enabled_tasks;
        let output = Arc::clone(&self.output);
        let steps = Arc::clone(&self.steps);
        let current_step = Arc::clone(&self.current_step);
        let running = Arc::clone(&self.running);
        let state = Arc::clone(&self.state);

        thread::spawn(move || {
            for (i, task) in tasks.iter().enumerate() {
                if !running.load(Ordering::Relaxed) {
                    let mut out = output.lock().unwrap();
                    out.push("\n=== Tasks cancelled ===".to_string());
                    break;
                }

                {
                    let mut s = steps.lock().unwrap();
                    if i < s.len() {
                        s[i].status = StepStatus::Running;
                    }
                }
                current_step.store(i, Ordering::Relaxed);

                let mut out = output.lock().unwrap();
                out.push(format!("\n=== {} ===", task.name));
                drop(out);

                let result = match task.id.as_str() {
                    "repos" => run_repos(&config, &password),
                    "dnf" => run_dnf(&config, &password),
                    "docker" => run_docker(&config, &password),
                    "flatpak" => run_flatpak(&config),
                    "homebrew" => run_homebrew(&config),
                    "opencode" => run_opencode(&config, &password),
                    "cargo" => run_cargo(&config),
                    "terra" => run_terra(&config, &password),
                    "stow" => run_stow_task(),
                    _ => Ok(()),
                };

                {
                    let mut s = steps.lock().unwrap();
                    if i < s.len() {
                        s[i].status = if result.is_ok() { StepStatus::Completed } else { StepStatus::Failed };
                    }
                }

                {
                    let mut out = output.lock().unwrap();
                    if result.is_ok() {
                        out.push(format!("✓ {} completed", task.name));
                    } else {
                        out.push(format!("✗ {} failed", task.name));
                    }
                }
            }

            if running.load(Ordering::Relaxed) {
                let mut out = output.lock().unwrap();
                out.push("\n=== All tasks complete! ===".to_string());
                out.push("Press ESC to return".to_string());
            }
            
            state.store(AppState::Done.as_usize(), Ordering::Relaxed);
        });
    }
}

fn run_repos(config: &Config, password: &str) -> Result<()> {
    let repo = &config.repositories;
    let cmd = format!(
        "echo '{}' | sudo -S dnf install -y {} {}",
        password, repo.rpm_fusion_free, repo.rpm_fusion_nonfree
    );
    execute_cmd(&cmd)?;
    
    let cmd = format!(
        "echo '{}' | sudo -S dnf config-manager addrepo --overwrite --from-repofile {}",
        password, repo.docker
    );
    execute_cmd(&cmd)?;
    
    let cmd = format!(
        "echo '{}' | sudo -S bash -c 'if ! rpm -q terra-release &>/dev/null; then dnf install --nogpgcheck --repofrompath terra,{} terra-release; fi'",
        password, repo.terra
    );
    execute_cmd(&cmd)?;
    
    Ok(())
}

fn run_dnf(config: &Config, password: &str) -> Result<()> {
    let cmd = config.commands.update.clone();
    let full_cmd = format!("echo '{}' | sudo -S {}", password, cmd);
    execute_cmd(&full_cmd)?;
    
    let pkg = &config.packages.dnf;
    let cmd = format!("echo '{}' | sudo -S dnf install -y {}", password, pkg.packages.join(" "));
    execute_cmd(&cmd)?;
    
    Ok(())
}

fn run_docker(config: &Config, password: &str) -> Result<()> {
    let pkg = &config.packages.docker;
    let cmd = format!("echo '{}' | sudo -S dnf install -y {}", password, pkg.packages.join(" "));
    execute_cmd(&cmd)?;
    
    if pkg.enable_service {
        let cmd = format!("echo '{}' | sudo -S systemctl enable --now docker", password);
        execute_cmd(&cmd)?;
    }
    
    Ok(())
}

fn run_flatpak(config: &Config) -> Result<()> {
    let pkg = &config.packages.flatpak;
    let cmd = format!(
        "flatpak remote-add --if-not-exists {} https://dl.flathub.org/repo/flathub.flatpakrepo",
        pkg.remote
    );
    execute_cmd(&cmd)?;
    
    let cmd = format!("flatpak install -y {}", pkg.apps.join(" "));
    execute_cmd(&cmd)?;
    
    Ok(())
}

fn run_homebrew(config: &Config) -> Result<()> {
    let pkg = &config.packages.homebrew;
    let cmd = format!("/bin/bash -c \"$(curl -fsSL {})\"", pkg.install_script);
    execute_cmd(&cmd)?;
    
    let cmd = format!(
        "eval \"$( {})\" && brew install {}",
        config.commands.shell_init,
        pkg.packages.join(" ")
    );
    execute_cmd(&cmd)?;
    
    Ok(())
}

fn run_opencode(config: &Config, password: &str) -> Result<()> {
    let pkg = &config.packages.opencode;
    let cmd = format!("echo '{}' | sudo -S dnf install -y {}", password, pkg.url);
    execute_cmd(&cmd)?;
    
    Ok(())
}

fn run_cargo(config: &Config) -> Result<()> {
    let pkg = &config.packages.cargo;
    let cmd = format!("cargo install {}", pkg.packages.join(" "));
    execute_cmd(&cmd)?;
    
    Ok(())
}

fn run_terra(config: &Config, password: &str) -> Result<()> {
    let pkg = &config.packages.terra;
    let cmd = format!("echo '{}' | sudo -S dnf install -y {}", password, pkg.packages.join(" "));
    execute_cmd(&cmd)?;
    
    Ok(())
}

fn run_stow_task() -> Result<()> {
    execute_cmd("mkdir -p $HOME/.local/bin && stow -R -t $HOME/ --dotfiles .")?;
    Ok(())
}

fn execute_cmd(cmd: &str) -> Result<()> {
    let output = Command::new("sh")
        .args(["-c", cmd])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()?;

    if !output.status.success() {
        return Err(color_eyre::eyre::anyhow!("Command failed"));
    }
    Ok(())
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
                                    app.start_tasks();
                                }
                                KeyCode::Backspace => {
                                    password_input.pop();
                                }
                                KeyCode::Esc => {
                                    app.set_state(AppState::Selection);
                                    password_input.clear();
                                }
                                KeyCode::Char(c) => {
                                    password_input.push(c);
                                }
                                _ => {}
                            }
                        }
                        AppState::Selection => {
                            match key.code {
                                KeyCode::Up => {
                                    if app.selected_index > 0 {
                                        app.selected_index -= 1;
                                    }
                                }
                                KeyCode::Down => {
                                    if app.selected_index < app.tasks.len() {
                                        app.selected_index += 1;
                                    }
                                }
                                KeyCode::Char(' ') => {
                                    if app.selected_index < app.tasks.len() {
                                        app.toggle_task();
                                    } else if app.selected_index == app.tasks.len() + 1 {
                                        app.set_state(AppState::Confirm);
                                        app.selected_index = 0;
                                        app.focus = Focus::Buttons;
                                    }
                                }
                                KeyCode::Enter => {
                                    if app.selected_index < app.tasks.len() {
                                        app.toggle_task();
                                    } else if app.selected_index == app.tasks.len() + 1 {
                                        app.set_state(AppState::Confirm);
                                        app.selected_index = 0;
                                        app.focus = Focus::Buttons;
                                    }
                                }
                                KeyCode::Esc => {
                                    return Ok(());
                                }
                                _ => {}
                            }
                        }
                        AppState::Confirm => {
                            match key.code {
                                KeyCode::Left | KeyCode::Right => {
                                    if app.focus == Focus::Buttons {
                                        app.selected_index = if app.selected_index == 0 { 1 } else { 0 };
                                    }
                                }
                                KeyCode::Enter => {
                                    if app.selected_index == 0 {
                                        app.running.store(true, Ordering::Relaxed);
                                        app.start_tasks();
                                    } else {
                                        app.set_state(AppState::Selection);
                                        app.selected_index = 0;
                                        app.focus = Focus::TaskList;
                                    }
                                }
                                KeyCode::Esc => {
                                    app.set_state(AppState::Selection);
                                    app.selected_index = 0;
                                    app.focus = Focus::TaskList;
                                }
                                _ => {}
                            }
                        }
                        AppState::Running => {
                            if key.code == KeyCode::Esc {
                                app.running.store(false, Ordering::Relaxed);
                            }
                        }
                        AppState::Done => {
                            if key.code == KeyCode::Esc {
                                app.set_state(AppState::Selection);
                                app.selected_index = 0;
                                app.focus = Focus::TaskList;
                                app.sudo_password.clear();
                                let mut out = app.output.lock().unwrap();
                                out.clear();
                                out.push("Welcome to dot-setup".to_string());
                            }
                        }
                    }
                }
            }
        }
        
        let current_state = app.get_state();
        if current_state == AppState::Running {
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
        .block(Block::default().borders(Borders::ALL).title("Setup"));
    frame.render_widget(title, chunks[0]);

    let state = app.get_state();

    match state {
        AppState::GettingPassword => {
            let prompt = Paragraph::new("Enter sudo password:")
                .style(Style::default().fg(Color::Yellow))
                .block(Block::default().borders(Borders::ALL).title("Password"));
            frame.render_widget(prompt, chunks[1]);
        }
        AppState::Selection => {
            let items: Vec<ListItem> = app
                .tasks
                .iter()
                .enumerate()
                .map(|(i, task)| {
                    let checkbox = if task.enabled { "[x]" } else { "[ ]" };
                    let content = format!("{} {}", checkbox, task.name);
                    let style = if i == app.selected_index {
                        Style::default().fg(Color::Yellow).add_modifier(ratatui::style::Modifier::BOLD)
                    } else {
                        Style::default().fg(Color::White)
                    };
                    ListItem::new(Span::styled(content, style))
                })
                .collect();

            let list = List::new(items)
                .block(Block::default().borders(Borders::ALL).title("Select tasks to run (Space to toggle)"))
                .style(Style::default().fg(Color::White));

            frame.render_widget(list, chunks[1]);
        }
        AppState::Confirm => {
            let enabled = app.get_enabled_tasks();
            let task_list: Vec<Line> = enabled.iter().map(|t| Line::from(t.name.as_str())).collect();
            
            let tasks_text = Paragraph::new(task_list)
                .block(Block::default().borders(Borders::ALL).title("Ready to run:"));
            frame.render_widget(tasks_text, chunks[1]);

            let confirm_text = if app.selected_index == 0 {
                Span::styled(" [Confirm] ", Style::default().fg(Color::Yellow).add_modifier(ratatui::style::Modifier::BOLD))
                    .to_owned()
            } else {
                Span::raw(" [Confirm] ")
            };
            let cancel_text = if app.selected_index == 1 {
                Span::styled(" [Cancel] ", Style::default().fg(Color::Yellow).add_modifier(ratatui::style::Modifier::BOLD))
                    .to_owned()
            } else {
                Span::raw(" [Cancel] ")
            };
            
            let buttons = Paragraph::new(Line::from(vec![confirm_text, Span::raw("   "), cancel_text]))
                .style(Style::default().fg(Color::White))
                .block(Block::default().borders(Borders::ALL).title("Confirm"));
            frame.render_widget(buttons, chunks[1]);
        }
        AppState::Running | AppState::Done => {
            let steps = app.steps.lock().unwrap();
            let current = app.current_step.load(Ordering::Relaxed) as usize;
            
            let stepper: Vec<Line> = steps
                .iter()
                .enumerate()
                .map(|(i, step)| {
                    let (icon, color) = match step.status {
                        StepStatus::Pending => ("○", Color::DarkGray),
                        StepStatus::Running => ("◐", Color::Yellow),
                        StepStatus::Completed => ("●", Color::Green),
                        StepStatus::Failed => ("✗", Color::Red),
                    };
                    let prefix = if i == current { "> " } else { "  " };
                    let style = Style::default().fg(color);
                    Line::from(vec![Span::raw(prefix), Span::styled(icon, style), Span::raw(format!(" {}", step.name))])
                })
                .collect();
            
            let stepper_widget = Paragraph::new(stepper)
                .block(Block::default().borders(Borders::ALL).title("Progress"));
            frame.render_widget(stepper_widget, chunks[1]);

            let output = app.output.lock().unwrap();
            let output_text: Vec<Line> = output
                .iter()
                .map(|s| Line::from(s.as_str()))
                .collect();
            let output_widget = Paragraph::new(output_text)
                .block(Block::default().borders(Borders::ALL).title("Output"))
                .scroll((app.scroll, 0));
            frame.render_widget(output_widget, chunks[1]);
        }
    }

    let help_text = match state {
        AppState::GettingPassword => "Type password | Enter Submit | Esc Cancel",
        AppState::Selection => "↑↓ Select | Space/Enter Toggle | Esc Exit",
        AppState::Confirm => "←→ Switch | Enter Confirm | Esc Back",
        AppState::Running => "Press ESC to cancel...",
        AppState::Done => "Press ESC to return",
    };
    let help = Paragraph::new(help_text)
        .style(Style::default().fg(Color::DarkGray))
        .block(Block::default().borders(Borders::ALL));
    frame.render_widget(help, chunks[2]);
}
