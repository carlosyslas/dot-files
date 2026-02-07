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
parser.add_argument('--flatpak-apps', action="store_true")
parser.add_argument('--paru-pkgs', action="store_true")
parser.add_argument('--update-mandb', action="store_true")
parser.add_argument('--all', action="store_true")

PACMAN_PKGS = [
    "wayland",
    "wayland-protocols",
    "libinput",
    "xorg-xwayland",
    "xdg-desktop-portal-gtk",
    "xdg-desktop-portal-gnome",
    "nautilus",
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
    "uv",
    "yt-dlp",
]

FLATPAK_APPS = [
    "com.brave.Browser",
    "com.github.tchx84.Flatseal",
    "app.zen_browser.zen",
    "com.sayonara_player.Sayonara",
    "org.qbittorrent.qBittorrent",
    "org.telegram.desktop",
    "io.github.kolunmi.Bazaar",
    "org.gimp.GIMP",
    "org.kde.krita",
    "org.inkscape.Inkscape",
    "io.gitlab.theevilskeleton.Upscaler",
    "org.localsend.localsend_app",
    "app.ytmdesktop.ytmdesktop",
    "org.audacityteam.Audacity",
    "org.kde.kdenlive",
    "com.ozmartians.VidCutter",
    "io.dbeaver.DBeaverCommunity",
    "io.dbeaver.DBeaverCommunity.Client.pgsql",
]

PARU_PKGS = [
    "noctalia-shell",
    "emacs-wayland",
]

def _sudo(cmd: list[str], sudo_pass: str):
    subprocess.run(
        ["sudo", "-S"] + cmd,
        input=sudo_pass,
        encoding="ascii",
    )


def _pacman_install(pkgs: list[str], sudo_pass: str):
    _sudo(["pacman", "-S", "--needed", "--noconfirm"] + pkgs, sudo_pass)


def update_system(sudo_pass: str):
    _sudo(["pacman", "-Su", "--noconfirm"], sudo_pass)


def install_pacman_pkgs(sudo_pass: str):
    _pacman_install(PACMAN_PKGS, sudo_pass)


def update_flatpak_apps():
    subprocess.run(["flatpak", "update", "-y"])


def install_flatpak_apps():
    subprocess.run(["flatpak", "install", "-y"] + FLATPAK_APPS)


def install_paru_pkgs(sudo_pass: str):
    subprocess.run(
        ["paru", "-S", "--needed", "--noconfirm"] + PARU_PKGS,
        input=sudo_pass,
        encoding="ascii",
    )


def update_manuals_db(sudo_pass: str):
    _sudo(["mandb"], sudo_pass)

def _banner(text: str):
    print("*" * (len(text) + 4))
    print(f"* {text} *")
    print("*" * (len(text) + 4))

def main():
    args = parser.parse_args()

    import getpass
    sudo_pass = getpass.getpass("Sudo password: ", echo_char="*")

    if args.system_upgrade or args.all:
        _banner("Updating System")
        update_system(sudo_pass)
    if args.pacman_pkgs or args.all:
        _banner("Install Pacman PKGs")
        install_pacman_pkgs(sudo_pass)
    if args.flatpak_apps or args.all:
        _banner("Install Flatpak Apps")
        install_flatpak_apps()
    if args.paru_pkgs or args.all:
        _banner("Install AUR PKGs")
        install_paru_pkgs(sudo_pass)
    if args.update_mandb or args.all:
        _banner("Update Manuals DB")
        update_manuals_db(sudo_pass)


if __name__ == "__main__":
    main()
