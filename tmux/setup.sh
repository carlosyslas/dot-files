TMUX_CONFIG=~/.tmux.conf

if [ ! -f $TMUX_CONFIG ]; then
    ln -s $INSTALL_DIR/tmux/tmux.conf $TMUX_CONFIG
fi
