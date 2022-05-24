use std::ffi::NulError;

#[derive(Debug)]
pub enum RReaderError {
    IO(std::io::Error),
    NoInputFile,
    MissingInput(&'static str),
    RePair(RePairError),
}

impl From<std::io::Error> for RReaderError {
    fn from(err: std::io::Error) -> Self {
        Self::IO(err)
    }
}

impl From<RePairError> for RReaderError {
    fn from(err: RePairError) -> Self {
        match err {
            RePairError::IO(io_err) => Self::IO(io_err),
            err => Self::RePair(err)
        }
    }
}

#[derive(Debug)]
pub enum RePairError {
    InvalidFileName(NulError),
    IO(std::io::Error)
}

impl From<NulError> for RePairError {
    fn from(err: NulError) -> Self {
        Self::InvalidFileName(err)
    }
}

impl From<std::io::Error> for RePairError {
    fn from(err: std::io::Error) -> Self {
        Self::IO(err)
    }
}
