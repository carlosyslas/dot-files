use clap::Parser;
use std::process::Command;

#[derive(Parser, Debug)]
#[command(name = "kde-run-or-raise")]
#[command(about = "Run or raise a KDE Plasma application", long_about = None)]
struct Args {
    /// Application ID (e.g., alacritty, firefox)
    #[arg(short, long)]
    app_id: String,

    /// Command to spawn if app is not running
    #[arg(short, long)]
    command: String,
}

fn main() {
    let args = Args::parse();

    if let Some(window_id) = find_window(&args.app_id) {
        println!("Found window {}, raising...", window_id);
        raise_window(&window_id);
    } else {
        println!("Window not found, launching...");
        launch_app(&args.command);
    }
}

fn find_window(app_id: &str) -> Option<String> {
    let output = Command::new("xdotool")
        .args(["search", "--class", app_id])
        .output()
        .ok()?;

    let windows = String::from_utf8_lossy(&output.stdout);
    windows.lines().next().map(|s| s.to_string())
}

fn raise_window(window_id: &str) {
    let _ = Command::new("xdotool")
        .args(["windowactivate", "--sync", window_id])
        .output();
}

fn launch_app(command: &str) {
    let _ = Command::new("bash")
        .args(["-c", command])
        .spawn();
}
