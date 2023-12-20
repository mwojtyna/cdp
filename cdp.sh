#!/bin/bash
cdp() {
	cd "$(cargo run -q -- ~/developer "$@")"
}
