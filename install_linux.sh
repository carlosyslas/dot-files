#!/bin/bash
APPS=(stow neovim jq bat exa ripgrep ranger emacs)

function install_ubuntu() {
    sudo apt-get update -y
    sudo apt-get install -y ${APPS[*]}

    mkdir -p ~/.local/bin
    ln -s /usr/bin/batcat ~/.local/bin/bat
}

function install_manjaro() {
    sudo pacman -S --noconfirm ${APPS[*]}
}


DISTRO=$(uname -r | awk '{ print tolower($0) }')
if [[ $DISTRO =~ "manjaro" ]]; then
    install_manjaro
else
    install_ubuntu
fi
