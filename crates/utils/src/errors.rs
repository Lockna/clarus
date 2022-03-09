//type Result<T> = std::result::Result<T, Error>;
use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum Error {
    IndexOfChunkNotFound,
}

#[derive(Debug, Clone, Copy)]
pub enum WaveError {
    RIFFStringNotFound,
    InvalidFileChunkSize,
    WavStringNotFound,
    FmtStringNotFound,
    InvalidFormatSize,
    InvalidByteRate,
    InvalidBlockAlign,
    InvalidDataSize,
    UnsupportedFormat,
    UnsupportedBitDepth,
    UnsupportedChannelNumber,
    ChunkNotFound,
}

impl fmt::Display for WaveError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl std::error::Error for WaveError {}