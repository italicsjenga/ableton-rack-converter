#!/bin/bash
# set -e 
cargo build --target=x86_64-pc-windows-gnu --target-dir=./build/release/windows --release
mkdir -p build/release/windows
cp build/windows/x86_64-pc-windows-gnu/release/converter.exe build/release/windows/ableton-rack-converter.exe
cp build/windows/x86_64-pc-windows-gnu/release/compressor.exe build/release/windows/xml-to-rack.exe
cp build/windows/x86_64-pc-windows-gnu/release/decompressor.exe build/release/windows/rack-to-xml.exe