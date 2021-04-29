#!/bin/bash
case "$(uname -s)" in
	Linux*) source install_linux.sh ;;
	Darwin*) source install_mac.sh ;;
esac

# Install neovim
source setup_neovim.sh

# Install tmux
stow -t $HOME tmux

# Install fish
fish setup_fish.fish
