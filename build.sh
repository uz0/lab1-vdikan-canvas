#!/bin/bash
set -e

RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release
mkdir -p out
mkdir -p neardev
cp target/wasm32-unknown-unknown/release/lab1_vdikan_canvas.wasm ./out/main.wasm
