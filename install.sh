#!/usr/bin/env bash

sudo dnf install https://mirrors.rpmfusion.org/free/fedora/rpmfusion-free-release-$(rpm -E %fedora).noarch.rpm https://mirrors.rpmfusion.org/nonfree/fedora/rpmfusion-nonfree-release-$(rpm -E %fedora).noarch.rpm
sudo dnf config-manager addrepo --from-repofile https://download.docker.com/linux/fedora/docker-ce.repo
sudo dnf update
sudo dnf upgrade -y
sudo dnf install -y vim stow alacritty git timeshift emacs yt-dlp imv mpv vlc zsh fastfetch
sudo dnf install -y docker-ce docker-ce-cli containerd.io docker-buildx-plugin docker-compose-plugin

flatpak remote-add --if-not-exists flathub https://dl.flathub.org/repo/flathub.flatpakrepo
flatpak install -y com.brave.Browser app.zen_browser.zen io.gitlab.theevilskeleton.Upscaler org.upscayl.Upscayl io.github.kolunmi.Bazaar org.qbittorrent.qBittorrent com.rafaelmardojai.Blanket com.github.johnfactotum.Foliate

/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
eval "$(/home/linuxbrew/.linuxbrew/bin/brew shellenv zsh)"
brew install anomalyco/tap/opencode
brew install starship
