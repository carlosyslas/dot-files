BASH_RC=~/.bashrc

if ! grep -q "source $INSTALL_DIR/bash/main.sh" $BASH_RC; then
    echo "source $INSTALL_DIR/bash/main.sh" >> $BASH_RC
fi
