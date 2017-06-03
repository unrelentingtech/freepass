# freepass-capi [![unlicense](https://img.shields.io/badge/un-license-green.svg?style=flat)](http://unlicense.org)

The free password manager for power users: C API for the core library.

## Building (for Android and iOS)

Get the Android NDK and [build a standalone toolchain](https://developer.android.com/ndk/guides/standalone_toolchain.html):

```bash
$ sudo chown $(whoami) /opt
$ brew install android-ndk
$ cd $(brew --prefix android-ndk)
$ build/tools/make_standalone_toolchain.py --stl libc++ --api 24 --force --arch arm   --install-dir /opt/ndk
$ build/tools/make_standalone_toolchain.py --stl libc++ --api 24 --force --arch arm64 --install-dir /opt/ndk64
$ build/tools/make_standalone_toolchain.py --stl libc++ --api 24 --force --arch x86   --install-dir /opt/ndkx86
```

(On FreeBSD, edit `make_standalone_toolchain.py`: `if platform.system() == 'Linux'` -> `if platform.system() == 'Linux' or platform.system() == 'FreeBSD'`)

Add the Rust targets:

```bash
$ rustup target add arm-linux-androideabi aarch64-linux-android i686-linux-android
```

Get submodules of this repo, fix the libsodium build scripts if building for iOS, build it:

```bash
$ git submodule update --init
$ cd libsodium
$ ./autogen.sh

$ vi dist-build/ios.sh # Remove "--enable-minimal"
$ ./dist-build/ios.sh

$ ../build-libsodium-android.sh
```

Finally, build this library:

```bash
$ export RUST_PREFIX=/opt/rust-arm # ... customize some variables if necessary (see the scripts)
# (if using `rustup`, something like `export RUST_PREFIX=~/.multirust/toolchains/stable-x86_64-apple-darwin`)
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
