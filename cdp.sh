#!/bin/bash

cdp() {
	cd "$(cargo run -q -- .. "$@")" || exit 1
}
