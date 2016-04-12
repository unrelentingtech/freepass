#!/bin/sh

: ${CARGO:="`which cargo`"}
: ${LIPO:="`which lipo`"}
: ${BUILD_MODE:="release"}
: ${RUST_PREFIX:="/opt/rust-arm"}
: ${SODIUM_ROOT:="../libsodium"}

export PATH="$RUST_PREFIX/bin:$PATH"
export DYLD_LIBRARY_PATH="$RUST_PREFIX/lib:$DYLD_LIBRARY_PATH"
# export PKG_CONFIG_ALLOW_CROSS=1
export SODIUM_LIB_DIR="$SODIUM_ROOT/libsodium-ios/lib" \

LIBNAME="libfreepass_capi.a"
CARGO_OPTS="-v"
if [[ "$BUILD_MODE" == "release" ]]; then
	CARGO_OPTS="$CARGO_OPTS --$BUILD_MODE"
fi
RUSTC_OPTS="--crate-type=staticlib"

set -e

echo "=> Building for armv7"
$CARGO rustc $CARGO_OPTS --target=armv7-apple-ios -- $RUSTC_OPTS

echo "=> Building for aarch64"
$CARGO rustc $CARGO_OPTS --target=aarch64-apple-ios -- $RUSTC_OPTS

## Uncomment i386 for 32-bit Simulator (or profiling o_0)
echo "=> Building for i386"
$CARGO rustc $CARGO_OPTS --target=i386-apple-ios -- $RUSTC_OPTS

echo "=> Building for x86_64"
$CARGO rustc $CARGO_OPTS --target=x86_64-apple-ios -- $RUSTC_OPTS

echo "=> Making universal library"
mkdir -p "target/universal/$BUILD_MODE"
$LIPO -create -output "target/universal/$BUILD_MODE/$LIBNAME" \
	"target/armv7-apple-ios/$BUILD_MODE/$LIBNAME" \
	"target/aarch64-apple-ios/$BUILD_MODE/$LIBNAME" \
	"target/x86_64-apple-ios/$BUILD_MODE/$LIBNAME" \
	"target/i386-apple-ios/$BUILD_MODE/$LIBNAME" \
