#!/bin/sh

: ${NDK_STANDALONE:="/opt/ndk"}
: ${ANDROID_HOME:="`brew --prefix android-sdk`"}
: ${NDK_HOME:="`brew --prefix android-ndk`"}
: ${CARGO:="`which cargo`"}
: ${RUST_PREFIX:="/opt/rust-arm"}
: ${SODIUM_ROOT:="../libsodium"}

export PATH="$RUST_PREFIX/bin:$NDK_STANDALONE/bin:$PATH"
export DYLD_LIBRARY_PATH="$RUST_PREFIX/lib:$NDK_STANDALONE/lib:$LD_LIBRARY_PATH"
export PKG_CONFIG_ALLOW_CROSS=1

CARGO_OPTS="-v"
RUSTC_OPTS="--crate-type=dylib"

echo "Android SDK: $ANDROID_HOME"
echo "Android NDK: $NDK_HOME"
echo "Android NDK standalone: $NDK_STANDALONE"
echo "Rust: $RUST_PREFIX"
echo "Cargo: $CARGO"
echo "libsodium: $SODIUM_ROOT"

echo "=> Building for arm"
PKG_CONFIG_PATH="$SODIUM_ROOT/libsodium-android-armv6/lib/pkgconfig" \
	$CARGO rustc $CARGO_OPTS --target=arm-linux-androideabi -- $RUSTC_OPTS

# echo "=> Building for aarch64"
# (cd "$SODIUM_ROOT" && ANDROID_NDK_HOME="$NDK_HOME" dist-build/android-armv8-a.sh)
# PKG_CONFIG_PATH="$SODIUM_ROOT/libsodium-android-armv8-a/lib/pkgconfig" \
# 	$CARGO rustc --target=aarch64-linux-android -v
