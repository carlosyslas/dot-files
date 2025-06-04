#!/usr/bin/env bash

stow -t $HOME/.config config

mkdir -p $HOME/.local/bin
stow -t $HOME/.local/bin bin
