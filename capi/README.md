# freepass-capi [![unlicense](https://img.shields.io/badge/un-license-green.svg?style=flat)](http://unlicense.org)

The free password manager for power users: C API for the core library.

## Building (for Android and iOS, from OS X)

Get the Android SDK:

```bash
$ brew install android-sdk
$ android
```

(Install all the usual things needed for building apps.)

Get the Android NDK and [build a standalone toolchain](https://developer.android.com/ndk/guides/standalone_toolchain.html):

```bash
$ sudo mkdir /opt/ndk /opt/ndk64
$ sudo chown $(whoami) /opt/ndk /opt/ndk64
$ brew install android-ndk
$ cd $(brew --prefix android-ndk)
$ build/tools/make-standalone-toolchain.sh --platform=android-21 --toolchain=arm-linux-androideabi-4.9 --install-dir=/opt/ndk
$ build/tools/make-standalone-toolchain.sh --platform=android-21 --toolchain=aarch64-linux-android-4.9 --install-dir=/opt/ndk64
```

Get the Rust source and build it:

```bash
$ sudo mkdir /opt/rust-arm
$ sudo chown $(whoami) /opt/rust-arm
$ git clone https://github.com/rust-lang/rust.git
$ cd rust
$ ./configure --prefix=/opt/rust-arm --target=arm-linux-androideabi,aarch64-linux-android,armv7-apple-ios,armv7s-apple-ios,aarch64-apple-ios,i386-apple-ios,x86_64-apple-ios,x86_64-apple-darwin \
  --disable-valgrind-rpass --disable-docs --disable-optimize-tests --disable-llvm-assertions --enable-fast-make --disable-jemalloc --enable-clang \
  --arm-linux-androideabi-ndk=/opt/ndk --aarch64-linux-android-ndk=/opt/ndk64
$ make -j4
$ make install
```

Get submodules of this repo, make some fixes to libsodium build scripts and build it:

```bash
$ git submodule update --init
$ cd libsodium
$ ./autogen.sh
$ vi dist-build/android-build.sh # Replace "android-16" with "android-21", remove "--enable-minimal"
$ vi dist-build/ios.sh # Remove "--enable-minimal"
$ ANDROID_NDK_HOME="$(brew --prefix android-ndk)" ./dist-build/android-arm.sh
$ ANDROID_NDK_HOME="$(brew --prefix android-ndk)" ./dist-build/android-armv8-a.sh
$ ./dist-build/ios.sh
```

Finally, build this library:

```bash
$ export RUST_PREFIX=/opt/rust-arm # ... customize some variables if necessary (see the scripts)
$ ./build-android.sh
$ ./build-ios.sh
```

If you have `dyld` errors, it's probably because of El Capitan's security features.
Make sure your `cargo` is up to date.
If you use multirust on El Capitan, try something like this:

```bash
$ CARGO=$(multirust which cargo) ./build-android.sh
$ CARGO=$(multirust which cargo) ./build-ios.sh
```

## Project-related stuff

See `../README.md`.
