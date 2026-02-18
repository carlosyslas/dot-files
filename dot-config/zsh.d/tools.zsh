# Bat
export BAT_THEME="gruvbox (Dark) (Medium)"

# Rust
if [ -f "$HOME/.cargo/env" ]; then
  source "$HOME/.cargo/env"
  . "$HOME/.cargo/env"
fi


# Fnm
FNM_PATH="$HOME/.local/share/fnm"
if [ -d "$FNM_PATH" ]; then
  export PATH="$FNM_PATH:$PATH"
  eval "$(fnm env)"
fi

# FZF
export FZF_DEFAULT_OPTS="$FZF_DEFAULT_OPTS \
  --highlight-line \
  --info=inline-right \
  --ansi \
  --layout=reverse \
  --border=none \
  --color=bg+:#283457 \
  --color=bg:#16161e \
  --color=border:#27a1b9 \
  --color=fg:#c0caf5 \
  --color=gutter:#16161e \
  --color=header:#ff9e64 \
  --color=hl+:#2ac3de \
  --color=hl:#2ac3de \
  --color=info:#545c7e \
  --color=marker:#ff007c \
  --color=pointer:#ff007c \
  --color=prompt:#2ac3de \
  --color=query:#c0caf5:regular \
  --color=scrollbar:#27a1b9 \
  --color=separator:#ff9e64 \
  --color=spinner:#ff007c \
"
# TODO: Add the fzf_preview command
export FZF_DEFAULT_COMMAND="fd --hidden --strip-cwd-prefix --exclude .git"
export FZF_CTRL_T_COMMAND="$FZF_DEFAULT_COMMAND"
#export FZF_CTRL_T_OPTS='--tmux center --layout=reverse --style minimal --preview="fish -c \'fzf_preview {}\'"'
export FZF_CTRL_R_OPTS='--tmux center --layout=reverse --style minimal'
#export FZF_ALT_C_OPTS='--tmux center --layout=reverse --style minimal --preview="fish -c \'fzf_preview {}\'"'
eval "$(fzf --zsh)"
