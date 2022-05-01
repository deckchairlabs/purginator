#!/bin/sh

set -eux

cargo build --target wasm32-unknown-unknown --release
wasm-bindgen --out-dir pkg --target deno ${CARGO_TARGET_DIR:-../target}/wasm32-unknown-unknown/release/purginator.wasm
wasm-opt -O3 -o pkg/purginator_bg.wasm pkg/purginator_bg.wasm