#!/usr/bin/env bash

mkdir -p $HOME/.local/bin

stow -R -t $HOME/ --dotfiles .
