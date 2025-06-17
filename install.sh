#!/usr/bin/env bash

sudo dnf install https://mirrors.rpmfusion.org/free/fedora/rpmfusion-free-release-$(rpm -E %fedora).noarch.rpm https://mirrors.rpmfusion.org/nonfree/fedora/rpmfusion-nonfree-release-$(rpm -E %fedora).noarch.rpm
sudo dnf update
sudo dnf upgrade -y
sudo dnf install -y vim neovim stow alacritty sway swaylock waybar wofi wlogout git mpd pcmanfm
