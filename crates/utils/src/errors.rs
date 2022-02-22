//type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, Copy)]
pub enum Error {
    InvalidWAVFile,
    IndexOfChunkNotFound,
}
