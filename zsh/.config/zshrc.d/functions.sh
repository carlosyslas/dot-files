#!/usr/bin/bash

function get_current_git_branch() {
    git rev-parse --abbrev-ref HEAD
}

function _yazi() {
    flatpak run io.github.sxyazi.yazi
}

