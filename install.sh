#!/bin/bash
case "$(uname -s)" in
	Linux*) source install_linux.sh ;;
	Darwin*) source install_mac.sh ;;
esac

# Install neovim
sh -c 'curl -fLo "${XDG_DATA_HOME:-$HOME/.local/share}"/nvim/site/autoload/plug.vim --create-dirs \
       https://raw.githubusercontent.com/junegunn/vim-plug/master/plug.vim'
stow -t $HOME nvim
nvim +PlugInstall +qall 

# Install tmux
stow -t $HOME tmux

# Install fish
stow -t $HOME fish
