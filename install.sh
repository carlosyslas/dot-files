#!/usr/bin/env bash
set -euo pipefail

echo "=== Adding RPM Fusion repositories ==="
sudo dnf install -y \
    https://download1.rpmfusion.org/free/fedora/rpmfusion-free-release-$(rpm -E %fedora).noarch.rpm \
    https://download1.rpmfusion.org/nonfree/fedora/rpmfusion-nonfree-release-$(rpm -E %fedora).noarch.rpm

echo "=== Adding Docker repository ==="
sudo dnf config-manager addrepo --from-repofile https://download.docker.com/linux/fedora/docker-ce.repo

echo "=== Adding Terra repository ==="
sudo dnf install --nogpgcheck --repofrompath 'terra,https://repos.fyralabs.com/terra$releasever' terra-release

echo "=== Updating system ==="
sudo dnf update -y
sudo dnf upgrade -y
echo "=== Installing core packages ==="
sudo dnf install -y \
    vim \
    stow \
    alacritty \
    git \
    timeshift \
    emacs \
    yt-dlp \
    imv \
    mpv \
    vlc \
    zsh \
    fastfetch \
    bat \
    ranger \
    cargo \
    jq \
    yq \
    fzf \
    ripgrep \
    fd-find \
    eza \
    bottom \
    starship \
    gh \
    curl \
    wget \
    unzip \
    tar \
    gzip \
    fuse \
    polkit \
    gnome-keyring \
    seahorse \
    libsodium \
    pkgconf-pkgs \
    pkgconfig

echo "=== Installing Docker ==="
sudo dnf install -y docker-ce docker-ce-cli containerd.io docker-buildx-plugin docker-compose-plugin
sudo systemctl enable --now docker

echo "=== Installing Flatpak apps ==="
flatpak remote-add --if-not-exists flathub https://dl.flathub.org/repo/flathub.flatpakrepo
flatpak install -y \
    com.brave.Browser \
    app.zen_browser.zen \
    io.gitlab.theevilskeleton.Upscaler \
    org.upscayl.Upscayl \
    io.github.kolunmi.Bazaar \
    org.qbittorrent.qBittorrent \
    com.rafaelmardojai.Blanket \
    com.github.johnfactotum.Foliate \
    org.telegram.desktop

echo "=== Installing Homebrew ==="
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
eval "$(/home/linuxbrew/.linuxbrew/bin/brew shellenv)"

echo "=== Installing Homebrew packages ==="
brew install anomalyco/tap/opencode
brew install starship hugo tlrc uv

echo "=== Installing OpenCode Desktop ==="
sudo dnf install -y https://opencode.ai/download/linux-x64-rpm

echo "=== Installing Cargo packages ==="
cargo install fnm bottom

echo "=== Installing Terra extras ==="
sudo dnf install -y terra-release-extras

echo "=== Setup complete! ==="
echo "Consider restarting or running: exec zsh"
