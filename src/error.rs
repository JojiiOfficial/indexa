#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Bitcode(bitcode::Error),
    Bincode(bincode::Error),
    OutOfBounds,
    Internal,
    DuplicateEntry,
    UnsupportedOperation,
}

impl From<bytestore::Error> for Error {
    fn from(value: bytestore::Error) -> Self {
        match value {
            bytestore::Error::Io(i) => Self::Io(i),
            bytestore::Error::Bitcode(b) => Self::Bitcode(b),
            bytestore::Error::Bincode(b) => Self::Bincode(b),
            bytestore::Error::OutOfBounds => Self::OutOfBounds,
            bytestore::Error::InvalidHeader
            | bytestore::Error::Initialization
            | bytestore::Error::UnexpectedValue
            | bytestore::Error::InvalidShift => Self::Internal,
            bytestore::Error::UnsupportedOperation => Self::UnsupportedOperation,
        }
    }
}

impl From<bitcode::Error> for Error {
    #[inline]
    fn from(value: bitcode::Error) -> Self {
        Self::Bitcode(value)
    }
}

impl From<bincode::Error> for Error {
    #[inline]
    fn from(value: bincode::Error) -> Self {
        Self::Bincode(value)
    }
}
