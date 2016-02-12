# freepass [![unlicense](https://img.shields.io/badge/un-license-green.svg?style=flat)](http://unlicense.org)

The free password manager for power users.

![X11 screencast gif](https://packett.cool/pub/screens/freepass-x11.gif)

## What's going on?

- A password manager.
- Based on the [Master Password algorithm], generates the same passwords as the Master Password apps.
- But wait, there's more! Why stop at passwords? It generates...
  - [Ed25519] digital signature keys for...
    - [OpenSSH]: Freepass adds private keys directly to a running ssh-agent & exports public keys in OpenSSH format!
    - [signify]: Freepass signs files & exports public keys in signify format!
    - *TODO* [SQRL]
  - Raw 256-bit keys for symmetric ciphers.
  - *TODO* [BIP39]-compatible passphrases.
- Yes, *all* of the above is *derived from your master password and full name*, you can always recover it by entering the same data!
- The generator settings (site names, counters) are stored in vault files:
  - Serialized into [CBOR].
  - Encrypted with NaCl secretbox for each entry + AES for the whole file.
  - (Keys are derived from the master password like the generated passwords.)
  - Every time you save a vault file, its size changes randomly. That's a feature. Some random junk is added to make it a bit harder to count how many passwords you have without opening the file.
- You can also *store* passwords and text in these vault files (for stuff that can't be generated).
- You can merge two vault files (e.g. from sync conflicts).
- You can import KeePass 2 (kdbx) files.

## How?

- Freepass is written in the [Rust] programming language and uses [libsodium] as the crypto library.
- Very modular code, easy to audit.
  - You can separately check that the `core` library does everything correctly, and that the user interface passes your data to the `core` library, not to an evil server.
- Some parts were written as completely separate Rust crates:
  - [secstr](https://github.com/valpackett/secstr): secure strings
  - [interactor](https://github.com/valpackett/interactor): user interface things for the `cli` version
  - [colorhash256](https://github.com/valpackett/colorhash256): [Chroma-Hash](https://github.com/mattt/Chroma-Hash/)-style color feedback for password input
  - [rusterpassword](https://github.com/valpackett/rusterpassword): the [Master Password algorithm] implementation
- Completely free software: public domain / [Unlicense].

[Master Password algorithm]: https://ssl.masterpasswordapp.com/algorithm.html
[Ed25519]: http://ed25519.cr.yp.to
[OpenSSH]: http://www.openssh.com
[signify]: http://www.tedunangst.com/flak/post/signify
[SQRL]: https://www.grc.com/sqrl/sqrl.htm
[BIP39]: https://github.com/bitcoin/bips/blob/master/bip-0039.mediawiki
[CBOR]: http://cbor.io
[Rust]: https://www.rust-lang.org
[libsodium]: https://download.libsodium.org/doc/
[Unlicense]: http://unlicense.org

## Where?

Freepass is (going to be) available on different platforms:

- `cli`: for UNIX-like systems
- *A desktop GUI and mobile apps will be available in the future.*

Each version has its own README!

## Contributing

By participating in this project you agree to follow the [Contributor Code of Conduct](http://contributor-covenant.org/version/1/2/0/).

[The list of contributors is available on GitHub](https://github.com/valpackett/freepass/graphs/contributors).

## License

This is free and unencumbered software released into the public domain.  
For more information, please refer to the `UNLICENSE` file or [unlicense.org](http://unlicense.org).
