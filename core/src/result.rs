use std::{io, str, string, result};
use cbor;
use keepass;

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
    StringCodecError(string::FromUtf8Error),
    StrCodecError(str::Utf8Error),
    KeepassReadError(keepass::OpenDBError),
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

impl From<string::FromUtf8Error> for Error {
    fn from(err: string::FromUtf8Error) -> Error {
        Error::StringCodecError(err)
    }
}

impl From<str::Utf8Error> for Error {
    fn from(err: str::Utf8Error) -> Error {
        Error::StrCodecError(err)
    }
}

impl From<keepass::OpenDBError> for Error {
    fn from(err: keepass::OpenDBError) -> Error {
        Error::KeepassReadError(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::OtherError(err)
    }
}

pub type Result<T> = result::Result<T, Error>;
