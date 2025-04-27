#!/usr/bin/bash

# Global
alias v="nvim"
alias ll="eza -l --icons --no-quotes --no-filesize --no-user --no-time --git"
alias ls="eza -l --icons --no-quotes --no-filesize --no-user --no-time --no-permissions --git"

# Git
alias g="git"
alias gco="git checkout"
alias gf="git fetch --prune"
alias gl="git pull --rebase"
alias glg="git log --graph"
alias gm="git merge"
alias gp="git push origin HEAD"
alias gpf='git push --force-with-lease origin $(get_current_git_branch)'
alias gpsup='git push --set-upstream origin $(get_current_git_branch)'
alias grb="git rebase"
alias gst="git status"

alias gg="lazygit"
