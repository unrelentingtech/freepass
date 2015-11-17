#!/bin/sh

: ${CARGO:="`which cargo`"}
: ${LIPO:="`which lipo`"}
: ${BUILD_MODE:="release"}
: ${RUST_PREFIX:="/opt/rust-arm"}
: ${SODIUM_ROOT:="../libsodium"}

mkdir -p $SODIUM_ROOT/libsodium-ios/lib/pkgconfig

cat <<EOF > $SODIUM_ROOT/libsodium-ios/lib/pkgconfig/libsodium.pc
prefix=$SODIUM_ROOT/libsodium-ios
exec_prefix=\${prefix}
libdir=\${exec_prefix}/lib
includedir=\${prefix}/include

Name: libsodium
Version: 1.0.6
Description: A portable, cross-compilable, installable, packageable fork of NaCl, with a compatible API.

Libs: -L\${libdir} -lsodium
Cflags: -I\${includedir}
EOF

export PATH="$RUST_PREFIX/bin:$PATH"
export DYLD_LIBRARY_PATH="$RUST_PREFIX/lib:$LD_LIBRARY_PATH"
export PKG_CONFIG_ALLOW_CROSS=1
export PKG_CONFIG_PATH="$SODIUM_ROOT/libsodium-ios/lib/pkgconfig"

LIBNAME="libfreepass_capi.a"
CARGO_OPTS="-v --$BUILD_MODE"
RUSTC_OPTS="--crate-type=staticlib"

echo "=> Building for armv7"
$CARGO rustc $CARGO_OPTS --target=armv7-apple-ios -- $RUSTC_OPTS

echo "=> Building for armv7s"
$CARGO rustc $CARGO_OPTS --target=armv7s-apple-ios -- $RUSTC_OPTS

echo "=> Building for aarch64"
$CARGO rustc $CARGO_OPTS --target=aarch64-apple-ios -- $RUSTC_OPTS

echo "=> Building for i386"
$CARGO rustc $CARGO_OPTS --target=i386-apple-ios -- $RUSTC_OPTS

echo "=> Building for x86_64"
$CARGO rustc $CARGO_OPTS --target=x86_64-apple-ios -- $RUSTC_OPTS

echo "=> Making universal library"
mkdir -p "target/universal/$BUILD_MODE"
$LIPO -create -output "target/universal/$BUILD_MODE/$LIBNAME" \
	"target/armv7-apple-ios/$BUILD_MODE/$LIBNAME" \
	"target/armv7s-apple-ios/$BUILD_MODE/$LIBNAME" \
	"target/aarch64-apple-ios/$BUILD_MODE/$LIBNAME" \
	"target/i386-apple-ios/$BUILD_MODE/$LIBNAME" \
	"target/x86_64-apple-ios/$BUILD_MODE/$LIBNAME"
