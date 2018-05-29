#!/bin/bash

INSTALL_DIR=~/.dot-files

# Clone or update
if [ -d $INSTALL_DIR ]; then
    (cd $INSTALL_DIR ; git pull)
else
    git clone git@github.com:carlosyslas/dot-files.git $INSTALL_DIR
fi

source bash/setup.sh
source tmux/setup.sh
source emacs.d/setup.sh
source node/setup.sh
