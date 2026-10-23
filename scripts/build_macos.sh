#!/usr/bin/env bash
# Builds a universal (Apple Silicon + Intel) Clip5.app and zips it into release/ with the
# version in the name. Needs cargo-bundle: cargo install cargo-bundle
set -euo pipefail

version=$(sed -n 's/^version *= *"\(.*\)"/\1/p' Cargo.toml | head -1)

rustup target add aarch64-apple-darwin x86_64-apple-darwin
cargo build --release --target aarch64-apple-darwin
cargo build --release --target x86_64-apple-darwin

# cargo-bundle creates the .app (Info.plist, icon) around a binary for this Mac only...
cargo bundle --release
app=target/release/bundle/osx/Clip5.app
exe=$(/usr/libexec/PlistBuddy -c "Print :CFBundleExecutable" "$app/Contents/Info.plist")

# ...so swap in a universal binary that runs on both kinds of Mac.
lipo -create -output "$app/Contents/MacOS/$exe" \
    "target/aarch64-apple-darwin/release/$exe" \
    "target/x86_64-apple-darwin/release/$exe"

# Ad-hoc sign the whole bundle. Without this, macOS on Apple Silicon reports a downloaded
# copy as "damaged" instead of offering Privacy & Security > Open Anyway.
codesign --force --deep --sign - "$app"

mkdir -p release
out="release/Clip5-${version}-macos.zip"
rm -f "$out"
ditto -c -k --keepParent "$app" "$out"
echo "Built $out"
