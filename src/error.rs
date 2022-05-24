#[derive(Debug)]
pub enum RReaderError {
    IO(std::io::Error),
    NoInputFile,
    MissingInput(&'static str),
}

impl From<std::io::Error> for RReaderError {
    fn from(err: std::io::Error) -> Self {
        Self::IO(err)
    }
}
