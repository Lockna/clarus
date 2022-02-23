use clarus_utils::file;
use std::path::Path;
use byteorder::ByteOrder;
use byteorder::LittleEndian;
use clarus_utils::pattern;
use std::str;

pub struct WavReader {

    pub data: Vec<u8>,
    cursor: usize

}

// Implement functions like read_u16_le (little endian or big endian)
// seek and so on
// set to position in data vector

impl WavReader {

    pub fn new(path: &Path) -> Self {
        WavReader {
            data: file::read_file(path).unwrap(),
            cursor: 0
        }
    }

    pub fn seek(&mut self, index: usize) {
        self.cursor = index;
    }

    pub fn seek_to_pattern(&mut self, pattern: &[u8]) {
        self.cursor = pattern::find_signature_index(&self.data, pattern).unwrap();
    }

    pub fn seek_forward(&mut self, inc_by: usize) {
        self.cursor += inc_by;
    }

    pub fn read_u16_le(&mut self) -> u16 {

        let ret = LittleEndian::read_u16(&self.data[self.cursor..self.cursor+2]);

        self.cursor += 2;

        ret

    }

    pub fn read_i16_le(&mut self) -> i16 {

        let ret = LittleEndian::read_i16(&self.data[self.cursor..self.cursor+2]);

        self.cursor += 2;

        ret

    }

    pub fn read_u32_le(&mut self) -> u32 {

        let ret = LittleEndian::read_u32(&self.data[self.cursor..self.cursor+4]);

        self.cursor += 4;

        ret

    }

    pub fn read_str(&mut self) -> &str {
        let ret = str::from_utf8(&self.data[self.cursor..self.cursor+4]).unwrap();
        self.cursor += 4;

        ret
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

}