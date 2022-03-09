use clarus_utils::file;
use std::path::Path;
use byteorder::ByteOrder;
use byteorder::LittleEndian;
use clarus_utils::{pattern, errors::WaveError};
use std::str;

const I24_MAX: i32 = 16777215;

pub struct WavReader {
    pub data: Vec<u8>,
    cursor: usize
}

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

    pub fn seek_to_chunk(&mut self, pattern: &[u8])-> Result<(), WaveError> {
        while self.cursor < self.data.len() {
            if &self.data[self.cursor .. self.cursor + pattern.len()] == pattern {
                return Ok(())
            } else {
                self.cursor += 4;
                self.cursor += self.read_u32_le() as usize;
            }
        }

        Err(WaveError::ChunkNotFound)
    }

    pub fn seek_forward(&mut self, inc_by: usize) {
        self.cursor += inc_by;
    }

    pub fn read_u8(&mut self) -> u8 {
        let ret = self.data[self.cursor];

        self.cursor += 1;

        ret
    }

    pub fn read_i8(&mut self) -> i8 {
        self.read_u8() as i8
    }

    pub fn read_u16_le(&mut self) -> u16 {

        let ret = LittleEndian::read_u16(&self.data[self.cursor..self.cursor+2]);

        self.cursor += 2;

        ret

    }

    pub fn read_i16_le(&mut self) -> i16 {

        //let ret = LittleEndian::read_i16(&self.data[self.cursor..self.cursor+2]);

        // in debug "my" version is taking half the time of byteorder crate
        // in release mine is about 5ms slower than byteorder

        let ret = (self.data[self.cursor+1] as i16) << 8 | self.data[self.cursor] as i16;

        self.cursor += 2;

        ret

    }

    pub fn read_i24_le(&mut self) -> i32 {

        let mut ret = ((self.data[self.cursor+1] as i32) << 8) | self.data[self.cursor] as i32;

        self.cursor += 3;

        ret |= (self.data[self.cursor-1] as i32) << 16 as i32;

        ret

    }

    pub fn read_u32_le(&mut self) -> u32 {

        let ret = LittleEndian::read_u32(&self.data[self.cursor..self.cursor+4]);

        self.cursor += 4;

        ret

    }

    pub fn read_i32_le(&mut self) -> i32 {
        let ret = LittleEndian::read_i32(&self.data[self.cursor..self.cursor+4]);

        self.cursor += 4;

        ret
    }

    pub fn read_f32_le(&mut self) -> f32 {
        let ret = LittleEndian::read_f32(&self.data[self.cursor..self.cursor+4]);

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

    pub fn reset(&mut self) {
        self.cursor = 0;
    }

    pub fn values_u8_as_f32(&mut self, data_size: u32) -> Vec<f32> {
        let mut values: Vec<f32> = Vec::new();

        for _ in 0..data_size {
            values.push(self.read_u8() as f32 / u8::MAX as f32);
        }

        values
    }

    pub fn values_i16_as_f32(&mut self, data_size: u32) -> Vec<f32> {
        let mut values: Vec<f32> = Vec::new();

        for _ in (0..data_size).step_by((16 / 8) as usize) {
            values.push(self.read_i16_le() as f32 / i16::MAX as f32);
        }

        values
    }

    pub fn values_i24_as_f32(&mut self, data_size: u32) -> Vec<f32> {
        let mut values: Vec<f32> = Vec::new();

        for _ in (0..data_size).step_by((24 / 8) as usize) {
            values.push(self.read_i24_le() as f32 / I24_MAX as f32);
        }

        values
    }

    pub fn values_i32_as_f32(&mut self, data_size: u32) -> Vec<f32> {
        let mut values: Vec<f32> = Vec::new();

        for _ in (0..data_size).step_by((32 / 8) as usize) {
            values.push(self.read_i32_le() as f32 / i32::MAX as f32);
        }

        values
    }

    pub fn values_f32(&mut self, data_size: u32) -> Vec<f32> {
        let mut values: Vec<f32> = Vec::new();

        for _ in (0..data_size).step_by((32 / 8) as usize) {
            values.push(self.read_f32_le());
        }

        values
    }

    pub fn values_f64(&mut self, data_size: u32) {
        // TODO: Figure out how to play f64 values
        // convert to f32 and lose precision?

    }

}