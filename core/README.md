# freepass-core [![unlicense](https://img.shields.io/badge/un-license-green.svg?style=flat)](http://unlicense.org)

The free password manager for power users: core library.

## What?

This is the internal library behind all of the user interfaces.

It defines all the data structures, serialization, encryption and output.

## How?

The `EncryptedVault` struct is what's stored on disk in [CBOR] format.

The `DecryptedVault` struct contains what's stored in the `ciphertext` field of the `EncryptedVault` in [CBOR] format, encrypted using AES-128-CTR (`DecryptedVaultData`).  
The key (`outer_key`) is the 16-byte BLAKE2b keyed hash of the string `freepass.outer`, using the master key as the key.

The `EncryptedEntry` struct is what's stored as values in the `entries` field of the `Vault`.

The `Entry` struct is what's stored in the `ciphertext` field of the `EncryptedEntry` in [CBOR] format, encrypted using NaCl secretbox (XSalsa20+Poly1305 authenticated encryption).  
The key for each entry is a Master Password site seed, using `entries_key` instead of the master key.  
`entries_key` is the 64-byte BLAKE2b keyed hash of the string `freepass.entries`, using the master key as the key.  

The `EntryMetadata` struct is what's stored in the `metadata` field of the `EncryptedEntry`.

[CBOR]: http://cbor.io

## Project-related stuff

See `../README.md`.
