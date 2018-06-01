#!/bin/bash

VSCODE="Visual Studio Code"

if [ ! -d "/Applications/$VSCODE.app" ]; then
    echo "Installing $VSCODE"
    self__download_zip https://go.microsoft.com/fwlink/?LinkID=620882
    self__install_dot_app "$VSCODE"
fi
