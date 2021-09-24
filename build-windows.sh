cargo build --target=x86_64-pc-windows-gnu --target-dir=./build/windows --release
mkdir -p build/release/windows
cp build/windows/x86_64-pc-windows-gnu/release/converter.exe build/release/windows/ableton-rack-converter.exe
cp build/windows/x86_64-pc-windows-gnu/release/compressor.exe build/release/windows/xml-to-compressed-rack.exe
