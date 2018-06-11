#!/bin/bash

VSCODE="Visual Studio Code"

if [ ! -d "/Applications/$VSCODE.app" ]; then
    self__banner "Installing $VSCODE"
    self__install_zip_app https://go.microsoft.com/fwlink/?LinkID=620882 "$VSCODE"
fi
