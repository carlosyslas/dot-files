#!/usr/bin/bash

alias ".."="cd .."
alias "..."="cd ../.."
alias "...."="cd ../../.."
alias "....."="cd ../../../.."
alias "......"="cd ../../../../.."
export CDPATH=".:$HOME:$HOME/src"
shopt -s autocd

eval "$(zoxide init bash)"
alias o='z $HOME/src/$(fd -t d -d 1 --exclude .git "" $HOME/src/ | awk -e "{ gsub(\"$HOME/src/\", \"\"); print \$1 }" | fzf)'

