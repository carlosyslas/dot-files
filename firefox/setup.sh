#!/bin/bash

FIREFOX="Firefox"

if [ ! -d "/Applications/$FIREFOX.app" ]; then
    self__banner "Installing $FIREFOX"
    self__install_dmg_app "https://download.mozilla.org/?product=firefox-latest-ssl&os=osx&lang=en-US" "$FIREFOX" "$FIREFOX"
fi
