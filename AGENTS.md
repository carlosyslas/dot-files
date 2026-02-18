# AGENTS.md - Development Guidelines

This is a dot-files repository for Fedora Linux, containing system configuration, shell scripts, and zsh configuration files.

## Repository Structure

```
dot-files/
├── dot-config/          # Stowed config files (~/.config/*)
│   └── zsh.d/           # Zsh configuration fragments
├── dot-bashrc.d/       # Bash configuration fragments
├── install.sh          # Fedora installer
├── stow.sh             # Symlink manager
└── README.md           # Setup instructions
```

## Repositories Enabled

- **RPM Fusion** - free and nonfree
- **Docker** - container runtime
- **Terra** - community repository (https://terra.fyralabs.com)
- **Homebrew** - Linuxbrew for additional packages
- **Flatpak** - for GUI applications

## Build/Lint/Test Commands

### Zsh Configuration

Lint all zsh files:
```bash
zsh -n ~/.config/zsh.d/*.zsh
```

Or from repo:
```bash
zsh -n /home/cya/src/dot-files/dot-config/zsh.d/*.zsh
```

### Bash Scripts

Check bash syntax:
```bash
bash -n stow.sh
bash -n install.sh
```

### Install/Deploy

Stow dot-files (creates symlinks in $HOME):
```bash
./stow.sh
```

Run full Fedora setup:
```bash
./install.sh
```

## Code Style Guidelines

### Zsh Configuration Files

**Location**: `dot-config/zsh.d/*.zsh`

- Use `.zsh` extension, not `.sh`
- Remove bash shebangs (`#!/usr/bin/bash`) from zsh files
- Use `$(command)` syntax, not backticks
- Use zsh-specific initializers:
  - `pyenv init - zsh` (not `- bash`)
  - `fzf --zsh` (not `--bash`)
- Always check for file existence before sourcing:
  ```zsh
  if [ -f "$HOME/.cargo/env" ]; then
    source "$HOME/.cargo/env"
  fi
  ```
- Use `[ -d ]` for directories, `[ -f ]` for files
- Use double quotes for variable expansions
- No shebang needed in zsh config fragments

### Shell Scripts (bash)

**Location**: Root level `*.sh` files

- Use `#!/usr/bin/env bash` shebang
- Use `set -euo pipefail` for safety
- Use `${var}` for variable expansion
- Quote all variable expansions: `"$var"`
- Use `local` for function-local variables
- Use `[[ ]]` for conditionals in bash

### Naming Conventions

- Config files: lowercase with meaningful names (`aliases.zsh`, `path.zsh`)
- Functions: snake_case in Python, lowercase in shell
- Variables: lowercase with underscores in Python, uppercase for constants in shell
- Constants: UPPER_CASE in shell scripts

### Error Handling

- Shell: Use `set -euo pipefail`
- Always check command exit status when needed
- Provide meaningful error messages
- Use conditional checks before destructive operations

### General Patterns

- Keep zsh config modular (separate files for aliases, path, tools, ui)
- Use descriptive variable names
- Comment complex or non-obvious configurations
- Group related settings together
- Use consistent indentation (2 or 4 spaces)

## Tools Used

- **Shell**: zsh (primary), bash (fallback)
- **Package Managers**: dnf, flatpak, brew, cargo, fnm
- **Tools**: starship, fzf, bat, eza, fd, ripgrep, bottom, fastfetch
- **WM**: niri, sway
- **Terminal**: alacritty, wezterm

## Testing Changes

After modifying zsh configs:
1. Run syntax check: `zsh -n ~/.config/zsh.d/tools.zsh`
2. Test in new shell: `zsh -i -c "source ~/.zshrc; echo 'OK'"`
3. Check for errors in new terminal session

After modifying install scripts:
1. Run syntax check: `bash -n install.sh`
2. Review changes carefully before executing (requires sudo)
