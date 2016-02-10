use std::{io, string, result};
use cbor;
use byteorder;

#[derive(Debug)]
pub enum Error {
    WrongEntriesKeyLength,
    WrongEntryNonceLength,
    WrongOuterNonceLength,
    WrongOuterKeyLength,
    WrongDerivedKeyLength,
    InappropriateFormat,
    SeedGenerationError,
    DecryptionError,
    CodecError(cbor::CborError),
    ByteCodecError(byteorder::Error),
    StringCodecError(string::FromUtf8Error),
    OtherError(io::Error),
    DataError,
    EntryNotFound,
    NotImplemented,
    NotAvailableOnPlatform,
    SSHAgentSocketNotFound,
}

impl From<cbor::CborError> for Error {
    fn from(err: cbor::CborError) -> Error {
        Error::CodecError(err)
    }
}

impl From<byteorder::Error> for Error {
    fn from(err: byteorder::Error) -> Error {
        Error::ByteCodecError(err)
    }
}

impl From<string::FromUtf8Error> for Error {
    fn from(err: string::FromUtf8Error) -> Error {
        Error::StringCodecError(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::OtherError(err)
    }
}

pub type Result<T> = result::Result<T, Error>;
