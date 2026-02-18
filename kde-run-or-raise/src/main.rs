use clap::Parser;
use std::process::Command;

#[derive(Parser, Debug)]
#[command(name = "kde-run-or-raise")]
#[command(about = "Run or raise a KDE Plasma application", long_about = None)]
struct Args {
    /// Application name (e.g., firefox, konsole, dolphin)
    #[arg(required = true)]
    app: String,

    /// Optional: executable path if different from app name
    #[arg(short, long)]
    exec: Option<String>,

    /// Switch to the app's desktop instead of raising
    #[arg(short, long)]
    switch_desktop: bool,
}

fn main() {
    let args = Args::parse();

    if let Some(window_id) = find_window(&args.app) {
        println!("Found window {}, raising...", window_id);
        raise_window(&window_id, args.switch_desktop);
    } else {
        println!("Window not found, launching...");
        launch_app(&args.app, args.exec.as_deref().unwrap_or(&args.app));
    }
}

fn find_window(app_name: &str) -> Option<String> {
    let output = Command::new("qdbus")
        .args(["org.kde.KWin", "/KWin", "org.kde.KWin.Windows"])
        .output();

    let Ok(output) = output else {
        return None;
    };

    let windows = String::from_utf8_lossy(&output.stdout);
    
    for line in windows.lines() {
        let window_id = line.trim();
        if window_id.is_empty() {
            continue;
        }

        if let Some(class) = get_window_property(window_id, "windowClass") {
            if class.to_lowercase().contains(&app_name.to_lowercase()) {
                return Some(window_id.to_string());
            }
        }
        
        if let Some(title) = get_window_property(window_id, "caption") {
            if title.to_lowercase().contains(&app_name.to_lowercase()) {
                return Some(window_id.to_string());
            }
        }
    }

    None
}

fn get_window_property(window_id: &str, property: &str) -> Option<String> {
    let output = Command::new("qdbus")
        .args([
            "org.kde.KWin",
            &format!("/KWin/Window/{}", window_id),
            "org.kde.KWin.Window",
            property,
        ])
        .output()
        .ok()?;

    let value = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if value.is_empty() {
        None
    } else {
        Some(value)
    }
}

fn raise_window(window_id: &str, switch_desktop: bool) {
    if switch_desktop {
        let _ = Command::new("qdbus")
            .args([
                "org.kde.KWin",
                &format!("/KWin/Window/{}", window_id),
                "org.kde.KWin.Window",
                "goDesktop",
            ])
            .output();
    }

    let _ = Command::new("qdbus")
        .args([
            "org.kde.KWin",
            &format!("/KWin/Window/{}", window_id),
            "org.kde.KWin.Window",
            "activate",
        ])
        .output();
}

fn launch_app(app_name: &str, exec: &str) {
    let output = Command::new("gtk-launch")
        .arg(app_name)
        .output();

    if output.is_err() || !output.unwrap().status.success() {
        let _ = Command::new("nohup")
            .arg(exec)
            .arg("&")
            .spawn();
    }
}
