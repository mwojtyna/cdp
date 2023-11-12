#!/bin/bash

# Add this function to .zshrc
psel() {
	cd "$(/home/mati/developer/psel/target/debug/psel)" || exit 1
}

# This is for debug only
psel
