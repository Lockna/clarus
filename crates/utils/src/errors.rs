//type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, Copy)]
pub enum Error {
    IndexOfChunkNotFound,
}

#[derive(Debug, Clone, Copy)]
pub enum InvalidWaveFile {
    RIFFStringNotFound,
    InvalidFileChunkSize,
    WavStringNotFound,
    FmtStringNotFound,
    InvalidByteRate,
    InvalidBlockAlign,
    InvalidDataSize,
}
