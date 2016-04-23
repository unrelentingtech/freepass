#!/bin/sh

: ${NDK_STANDALONE:="/opt/ndk"}
: ${NDK_STANDALONE_ARM64:="/opt/ndk64"}
: ${NDK_STANDALONE_X86:="/opt/ndkx86"}

build() {
	echo "=> Building for ${TARGET_ARCH}"
	PATH="$PATH:${TOOLCHAIN_DIR}/bin"
	PREFIX="`pwd`/libsodium-android-${TARGET_ARCH}"
	./configure --disable-soname-versions --host="${HOST_COMPILER}" \
		--prefix="${PREFIX}" --with-sysroot="${TOOLCHAIN_DIR}/sysroot" && \
		make clean && \
		make -j3 install
}

export TARGET_ARCH=i686 ARCH=x86 HOST_COMPILER=i686-linux-android TOOLCHAIN_DIR="$NDK_STANDALONE_X86"
export CFLAGS="-Os -march=${TARGET_ARCH}"
build

export TARGET_ARCH=armv6 ARCH=arm HOST_COMPILER=arm-linux-androideabi TOOLCHAIN_DIR="$NDK_STANDALONE"
export CFLAGS="-Os -mthumb -marm -march=${TARGET_ARCH}"
build

export TARGET_ARCH=armv7-a ARCH=arm HOST_COMPILER=arm-linux-androideabi TOOLCHAIN_DIR="$NDK_STANDALONE"
export CFLAGS="-Os -mfloat-abi=softfp -mfpu=vfpv3-d16 -mthumb -marm -march=${TARGET_ARCH}"
build

export TARGET_ARCH=armv8-a ARCH=arm64 HOST_COMPILER=aarch64-linux-android TOOLCHAIN_DIR="$NDK_STANDALONE_ARM64"
export CFLAGS="-Os -march=${TARGET_ARCH}"
build
