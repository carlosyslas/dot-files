#!/usr/bin/env python3

import argparse
import subprocess

parser = argparse.ArgumentParser(
    prog="install",
    description="Install system packages on Arch BTW",
)

parser.add_argument('--keyring', action="store_true")
parser.add_argument('--system-upgrade', action="store_true")
parser.add_argument('--pacman-pkgs', action="store_true")

PACMAN_PKGS = [
    "wayland",
    "wayland-protocols",
    "libinput",
    "xorg-xwayland",
    "pipewire",
    "pipewire-audio",
    "pipewire-pulse",
    "wireplumber",
    "seatd",
    "dbus",
    "niri",
    "ly",
    "alacritty",
    "flatpak",
    "git",
    "base-devel",
    "acpi",
    "ttf-meslo-nerd",
    "podman",
    "podman-compose",
    "distrobox",
    "cava",
    "pcmanfm-qt",
    "lxqt-archiver",
    "unzip",
    "unrar",
    "lxqt-policykit",
    "man-pages",
    "man-db",
    "wget",
    "papirus-icon-theme",
    "swaybg",
    "fastfetch",
    "bottom",
    "imv",
    "mpv",
    "vlc",
    "openssh",
    "wl-clipboard",
]

FLATPAK_APPS = [
    "com.brave.Browser",
    "com.github.tchx84.Flatseal",
    "app.zen_browser.zen",
    "com.sayonara_player.Sayonara",
    "org.qbittorrent.qBittorrent",
    "flathub org.telegram.desktop",
    "io.github.kolunmi.Bazaar",
    "org.gimp.GIMP",
    "org.kde.krita",
    "org.inkscape.Inkscape",
    "io.gitlab.theevilskeleton.Upscaler",
    "org.localsend.localsend_app",
]

def _sudo(cmd: list[str], sudo_pass: str):
    subprocess.run(
        ["sudo", "-S"] + cmd,
        input=sudo_pass,
        encoding="ascii",
    )


def _pacman_install(pkgs: list[str], sudo_pass: str):
    _sudo(["pacman", "-S", "--needed", "--noconfirm"] + pkgs)


def update_system(sudo_pass: str):
    _sudo(["pacman", "-Su", "--noconfirm"])


def install_pacman_pkgs(sudo_pass: str):
    _pacman_install(PACMAN_PKGS, sudo_pass)


def update_flatpak_apps():
    subprocess.run(["flatpak update"])


def install_flatpak_apps():
    subprocess.run(["flatpak install"] + FLATPAK_APPS)


def main():
    args = parser.parse_args()
    print(args)
    exit(1)

    import getpass
    sudo_pass = getpass.getpass("Sudo password: ", echo_char="*")

    update_system(sudo_pass)
    install_pacman_pkgs(sudo_pass)


if __name__ == "__main__":
    main()
