#!/bin/bash

mkdir -p dist

# Build the wasm binary
build() {
    cargo build --target wasm32-unknown-unknown --release
    cp target/wasm32-unknown-unknown/release/TwoZeroFourEight.wasm dist/TwoZeroFourEight.wasm
    cp index.html dist/
}

build
basic-http-server dist --port 8080
SERVER_PID=$!

cleanup() {
    kill $SERVER_PID
    exit 0
}

trap cleanup SIGINT

find src index.html Cargo.toml -type f | entr -r bash -c '
    build
'

cleanup
