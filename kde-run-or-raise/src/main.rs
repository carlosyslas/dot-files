use clap::Parser;
use std::process::Command;

#[derive(Parser, Debug)]
#[command(name = "kde-run-or-raise")]
#[command(about = "Run or raise a KDE Plasma application", long_about = None)]
struct Args {
    /// Application ID (e.g., firefox, alacritty)
    #[arg(short, long)]
    app_id: String,

    /// Command to spawn if app is not running
    #[arg(short, long)]
    command: String,
}

fn main() {
    let args = Args::parse();

    if raise_with_kstart(&args.app_id) {
        println!("Raised existing window");
    } else {
        println!("Window not found, launching...");
        launch_app(&args.command);
    }
}

fn raise_with_kstart(app_id: &str) -> bool {
    let output = Command::new("kstart")
        .args(["--activate", "-a", app_id])
        .output();

    output.map(|o| o.status.success()).unwrap_or(false)
}

fn launch_app(command: &str) {
    let _ = Command::new("bash")
        .args(["-c", command])
        .spawn();
}
