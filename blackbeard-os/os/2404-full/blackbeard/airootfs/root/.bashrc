#!/bin/bash

# Set PATH
export PATH="$HOME/bin:$PATH"

# Aliases
alias ll='ls -alF'
alias la='ls -A'
alias l='ls -CF'

# Load .profile if it exists
if [ -f "$HOME/.profile" ]; then
	source "$HOME/.profile"
fi

# Autorun installer
# if [ -f "usr/local/bin/blackbeard_installer" ]; then
# 	/usr/local/bin/blackbeard_installer
# fi
