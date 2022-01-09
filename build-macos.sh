#!/bin/bash
# set -e 
cargo build --target=x86_64-apple-darwin --target-dir=./build/macos --release
mkdir -p build/release/macos
cp build/macos/x86_64-apple-darwin/release/converter build/release/macos/ableton-rack-converter
cp build/macos/x86_64-apple-darwin/release/compressor build/release/macos/xml-to-rack
cp build/macos/x86_64-apple-darwin/release/decompressor build/release/macos/rack-to-xml