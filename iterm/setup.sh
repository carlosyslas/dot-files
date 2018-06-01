#!/bin/bash

ITERM="iTerm"

if [ ! -d "/Applications/$ITERM.app" ]; then
    echo "Installing $ITERM"
    self__download_zip https://iterm2.com/downloads/stable/latest
    self__install_dot_app "$ITERM"
fi
