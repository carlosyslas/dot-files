#!/bin/bash

ITERM="iTerm"

if [ ! -d "/Applications/$ITERM.app" ]; then
    self__banner "Installing $ITERM"
    self__install_zip_app https://iterm2.com/downloads/stable/latest "$ITERM"
fi
