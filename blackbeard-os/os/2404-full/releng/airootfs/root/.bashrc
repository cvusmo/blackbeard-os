#!/bin/bash

# Set PATH
export PATH="$HOME/bin:$PATH"

# Aliases ? not usre if that is spelled right but whatever
alias ll='ls -alF'
alias la='ls -A'
alias l='ls -CF'

# Load .profile if it exists
if [ -f "$HOME/.profile" ]; then
	source "$HOME/.profile"
fi
