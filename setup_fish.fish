#!/usr/bin/fish
stow -t $HOME fish
curl -sL https://git.io/fisher | source && fisher install jorgebucaran/fisher

fisher install jethrokuan/z
fisher install dracula/fish
fisher install simnalamburt/shellder
