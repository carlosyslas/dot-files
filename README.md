# My dot-files

## Quick Install

```bash
curl -s https://raw.githubusercontent.com/carlosyslas/dot-files/v0.2.1/dot-setup/install.sh | bash
```

This will:
1. Download and run the dot-setup TUI application
2. Allow you to select which tasks to run
3. Execute the selected installation tasks

## Manual Setup

### 1. Clone the repository
Using `ssh`:
```bash
git clone ssh://git@github.com/carlosyslas/dot-files.git
```
Using `https`:
```bash
git clone https://github.com/carlosyslas/dot-files.git
```

### 2. Run the setup TUI
```bash
cd dot-files
./dot-setup/target/release/dot-setup
```

Or use the install script:
```bash
./dot-setup/install.sh
```

### 3. "Stow" files
```bash
./stow.sh
```

## Features

The dot-setup TUI application allows you to:
- Toggle individual tasks on/off
- See real-time progress with stepper
- Install: Repositories, System packages, Docker, Flatpak apps, Homebrew, OpenCode, Cargo packages, Terra extras
- Stow dotfiles

## Building from Source

```bash
cd dot-setup
cargo build --release
```
