#!/bin/sh

: ${NDK_STANDALONE:="/opt/ndk"}
: ${NDK_STANDALONE_ARM64:="/opt/ndk64"}
: ${NDK_STANDALONE_X86:="/opt/ndkx86"}
: ${BUILD_MODE:="release"}
: ${RUST_PREFIX:="/opt/rust-arm"}
: ${SODIUM_ROOT:="../libsodium"}

export PATH="$RUST_PREFIX/bin:$PATH"
export DYLD_LIBRARY_PATH="$RUST_PREFIX/lib:$DYLD_LIBRARY_PATH:$LD_LIBRARY_PATH"
export PKG_CONFIG_ALLOW_CROSS=1

: ${CARGO:="`which cargo`"}

CARGO_OPTS="-v -j 4"
if [[ "$BUILD_MODE" == "release" ]]; then
	CARGO_OPTS="$CARGO_OPTS --$BUILD_MODE"
fi
RUSTC_OPTS="--crate-type=dylib"

echo "Android NDK standalone: $NDK_STANDALONE"
echo "Rust: $RUST_PREFIX"
echo "Cargo: $CARGO"
echo "libsodium: $SODIUM_ROOT"

set -e

echo "=> Building for arm"
DYLD_LIBRARY_PATH="$NDK_STANDALONE/lib:$DYLD_LIBRARY_PATH" \
LD_LIBRARY_PATH="$NDK_STANDALONE/lib:$LD_LIBRARY_PATH" \
PATH="$NDK_STANDALONE/arm-linux-androideabi/bin:$NDK_STANDALONE/bin:$PATH" \
SODIUM_LIB_DIR="$SODIUM_ROOT/libsodium-android-armv6/lib" \
	$CARGO rustc $CARGO_OPTS --target=arm-linux-androideabi -- $RUSTC_OPTS -C linker=arm-linux-androideabi-gcc -C ar=arm-linux-androideabi-ar

echo "=> Building for aarch64"
DYLD_LIBRARY_PATH="$NDK_STANDALONE_ARM64/lib:$DYLD_LIBRARY_PATH" \
LD_LIBRARY_PATH="$NDK_STANDALONE_ARM64/lib:$LD_LIBRARY_PATH" \
PATH="$NDK_STANDALONE_ARM64/aarch64-linux-android/bin:$NDK_STANDALONE_ARM64/bin:$PATH" \
SODIUM_LIB_DIR="$SODIUM_ROOT/libsodium-android-armv8-a/lib" \
	$CARGO rustc $CARGO_OPTS --target=aarch64-linux-android -- $RUSTC_OPTS -C linker=aarch64-linux-android-gcc -C ar=aarch64-linux-android-ar

echo "=> Building for i686"
DYLD_LIBRARY_PATH="$NDK_STANDALONE_X86/lib:$DYLD_LIBRARY_PATH" \
LD_LIBRARY_PATH="$NDK_STANDALONE_X86/lib:$LD_LIBRARY_PATH" \
PATH="$NDK_STANDALONE_X86/i686-linux-android/bin:$NDK_STANDALONE_X86/bin:$PATH" \
SODIUM_LIB_DIR="$SODIUM_ROOT/libsodium-android-i686/lib" \
	$CARGO rustc $CARGO_OPTS --target=i686-linux-android -- $RUSTC_OPTS -C linker=i686-linux-android-gcc -C ar=i686-linux-android-ar

echo "=> Moving files"
mkdir -p "target/android-all/$BUILD_MODE/x86"
cp "target/i686-linux-android/$BUILD_MODE/libfreepass_capi.so" "target/android-all/$BUILD_MODE/x86"
mkdir -p "target/android-all/$BUILD_MODE/armeabi"
cp "target/arm-linux-androideabi/$BUILD_MODE/libfreepass_capi.so" "target/android-all/$BUILD_MODE/armeabi"
mkdir -p "target/android-all/$BUILD_MODE/armeabi-v7a"
cp "target/arm-linux-androideabi/$BUILD_MODE/libfreepass_capi.so" "target/android-all/$BUILD_MODE/armeabi-v7a"
mkdir -p "target/android-all/$BUILD_MODE/arm64-v8a"
cp "target/aarch64-linux-android/$BUILD_MODE/libfreepass_capi.so" "target/android-all/$BUILD_MODE/arm64-v8a"
