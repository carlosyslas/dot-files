function o
    z $HOME/src/$(fd -t d -d 1 --exclude .git "" $HOME/src/ | awk -e "{ gsub(\"$HOME/src/\", \"\"); print \$1 }" | fzf)
end
