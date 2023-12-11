#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Bitcode(bitcode::Error),
    OutOfBounds,
    Internal,
    DuplicateEntry,
}

impl From<mapstore::Error> for Error {
    fn from(value: mapstore::Error) -> Self {
        match value {
            mapstore::Error::Io(i) => Self::Io(i),
            mapstore::Error::Bitcode(b) => Self::Bitcode(b),
            mapstore::Error::OutOfBounds => Self::OutOfBounds,
            mapstore::Error::InvalidHeader => Self::Internal,
            mapstore::Error::InvalidShift => Self::Internal,
        }
    }
}

impl From<bitcode::Error> for Error {
    #[inline]
    fn from(value: bitcode::Error) -> Self {
        Self::Bitcode(value)
    }
}
