#!/bin/bash
mkdir -p ~/src
git clone git@github.com:carlosyslas/dot-files.git ~/src/dot-files
cd ~/src/dot-files/

case "$(uname -s)" in
	Linux*) source install_linux.sh ;;
	Darwin*) source install_mac.sh ;;
esac

# Install fnm
curl -fsSL https://fnm.vercel.app/install | bash
source /home/carlos/.zshrc

# Install neovim
source setup_neovim.sh

# Install tmux
stow -t $HOME tmux

# Install fish
#fish setup_fish.fish

# Install alacritty
stow -t $HOME alacritty

# Install emacs
stow -t $HOME emacs
