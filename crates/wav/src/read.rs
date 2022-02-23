use clarus_utils::file;
use std::path::Path;

pub struct WavReader {

    pub data: Vec<u8>,
    cursor: usize

}

// Implement functions like read_u16_le (little endian or big endian)
// seek and so on

impl WavReader {

    pub fn new(path: &Path) -> WavReader {
        WavReader {
            data: file::read_file(path).unwrap(),
            cursor: 0
        }
    }

}