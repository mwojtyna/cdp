#!/bin/bash

psel() {
	cd "$(cargo run -q ..)" || exit 1
}
