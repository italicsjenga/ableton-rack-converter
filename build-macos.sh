cargo build --target-dir=./build/macos --release
mkdir -p build/release/macos
cp build/macos/release/converter build/release/macos/ableton-rack-converter
cp build/macos/release/compressor build/release/macos/xml-to-compressed-rack
