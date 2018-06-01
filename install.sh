#!/bin/bash

INSTALL_DIR=~/.dot-files

# Clone or update
if [ -d $INSTALL_DIR ]; then
    (cd $INSTALL_DIR ; git pull)
else
    git clone git@github.com:carlosyslas/dot-files.git $INSTALL_DIR
fi

self__setup() {
    source $INSTALL_DIR/$1/setup.sh
}

self__setup bash
self__setup tmux
self__setup homebrew
self__setup emacs.d
self__setup node
