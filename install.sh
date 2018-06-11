#!/bin/bash

INSTALL_DIR=~/.dot-files
TMP_DIR=$(mktemp -d)

self__banner() {
    LEN=${#1}
    LINE="+-$(seq -f "-" -s '' $LEN)-+"
    echo $LINE
    echo "| $1 |"
    echo $LINE
}

# Clone or update
if [ -d $INSTALL_DIR ]; then
    (cd $INSTALL_DIR ; git pull)
else
    git clone git@github.com:carlosyslas/dot-files.git $INSTALL_DIR
fi

self__setup() {
    source $INSTALL_DIR/$1/setup.sh
}

self__install_zip_app() {
    FILE_NAME=$(date +%s | md5)
    curl $1 -L --out $TMP_DIR/$FILE_NAME.zip
    unzip -q $TMP_DIR/$FILE_NAME.zip -d $TMP_DIR
    sudo mv "$TMP_DIR/$2.app" "/Applications/$2.app"
}

self__install_dmg_app() {
    FILE_NAME=$(date +%s | md5)
    echo "curl $1 -L --out $TMP_DIR/$FILE_NAME.dmg"
    curl $1 -L --out $TMP_DIR/$FILE_NAME.dmg
    hdiutil attach $TMP_DIR/$FILE_NAME.dmg
    sudo cp -r "/Volumes/$2/$3.app" "/Applications/$3.app"
    hdiutil unmount /Volumes/"$2"
}

#self__setup bash
#self__setup tmux
#self__setup homebrew
#self__setup emacs.d
#self__setup node
self__setup vscode
self__setup iterm
self__setup firefox
