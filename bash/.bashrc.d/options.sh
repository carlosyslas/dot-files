#!/usr/bin/bash

set -o vi

export EDITOR="nvim"

export HISTCONTROL=ignoreboth
export HISTSIZE=1000
export HISTFILESIZE=1000

shopt -s histappend

export MANPAGER="sh -c 'sed -e s/.\\\\x08//g | bat -l man -p'"

