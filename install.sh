#!/bin/bash

INSTALL_DIR=~/.dot-files

# Clone or update
if [ -d $INSTALL_DIR ]; then
    (cd $INSTALL_DIR ; git pull)
else
    git clone git@github.com:carlosyslas/dot-files.git $INSTALL_DIR
fi

# Setup bash
if ! grep -q "source $INSTALL_DIR/bash/main.sh" ~/.bashrc; then
    echo "source $INSTALL_DIR/bash/main.sh" >> ~/.bashrc
fi

# Tmux
if [ ! -f ~/.tmux.conf ]; then
    ln -s $INSTALL_DIR/tmux/tmux.conf ~/.tmux.conf
fi

#ln -s $INSTALL_DIR/bash/
