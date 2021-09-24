rm -rf build/release
mkdir -p build/release
./build-macos.sh
./build-windows.sh
cd build/release
zip -j macos-ableton-rack-converter.zip macos/*
zip -j windows-ableton-rack-converter.zip windows/*