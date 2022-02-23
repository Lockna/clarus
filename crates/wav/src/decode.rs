use byteorder::ByteOrder;
use byteorder::LittleEndian;
use clarus_utils::errors::Error;
use clarus_utils::pattern;
use std::str;
use crate::read::WavReader;
use std::path::Path;

pub struct WavDecoder {

    reader: WavReader,
    format: u16,
    channels: u16,
    sample_rate: u32,
    bitdepth: u16
    // samples: decoded data

}

impl WavDecoder {

    pub fn new(path: &Path) -> WavDecoder {
        WavDecoder {
            reader: WavReader::new(path),
            format: 0,
            channels: 0,
            sample_rate: 0,
            bitdepth: 0
        }
    }

    pub fn decode(&self) -> Result<(u32, Vec<i16>), Error> {
        // Files from qobuz use id3v2

        let contents = &self.reader.data;

        let fmt_start = pattern::find_signature_index(&contents, b"fmt ").unwrap();
    
        //println!("{:?}", fmt_start);
    
        let riff_str = str::from_utf8(&contents[0..4]).unwrap();
    
        if riff_str != "RIFF" {
            return Err(Error::InvalidWAVFile);
        }
    
        //println!("{}", riff_str);
    
        let chunk_size = LittleEndian::read_u32(&contents[4..8]);
    
        if chunk_size != contents.len() as u32 - 8 {
            return Err(Error::InvalidWAVFile);
        }
    
        println!("{}", contents.len());
    
        println!("{}", chunk_size);
    
        let wave_str = str::from_utf8(&contents[8..12]).unwrap();
    
        if wave_str != "WAVE" {
            return Err(Error::InvalidWAVFile);
        }
    
        //println!("{}", wave_str);
    
        let fmt_str = str::from_utf8(&contents[fmt_start..fmt_start + 4]).unwrap();
    
        //println!("{}", fmt_str);
    
        let fmt_size = LittleEndian::read_u32(&contents[fmt_start + 4..fmt_start + 8]);
    
        //println!("fmt_size: {}", fmt_size);
    
        let fmt_format_code = LittleEndian::read_u16(&contents[fmt_start + 8..fmt_start + 10]);
    
        println!("fmt_format_code: {}", fmt_format_code);
    
        let fmt_num_channels = LittleEndian::read_u16(&contents[fmt_start + 10..fmt_start + 12]);
    
        println!("fmt_num_channels: {}", fmt_num_channels);
    
        let fmt_sample_rate = LittleEndian::read_u32(&contents[fmt_start + 12..fmt_start + 16]);
    
        println!("fmt_sample_rate: {}", fmt_sample_rate);
    
        let fmt_byte_rate = LittleEndian::read_u32(&contents[fmt_start + 16..fmt_start + 20]);
    
        println!("fmt_byte_rate: {}", fmt_byte_rate);
    
        let fmt_block_align = LittleEndian::read_u16(&contents[fmt_start + 20..fmt_start + 22]);
    
        println!("fmt_block_align: {}", fmt_block_align);
    
        let fmt_bits_sample = LittleEndian::read_u16(&contents[fmt_start + 22..fmt_start + 24]);
    
        println!("fmt_bits_sample: {}", fmt_bits_sample);
    
        // TODO: search properly for data begin
        let data_chunk_begin = fmt_start + 24;
    
        let data_str = str::from_utf8(&contents[data_chunk_begin..data_chunk_begin + 4]).unwrap();
    
        println!("{}", data_str);
    
        let data_size = LittleEndian::read_u32(&contents[data_chunk_begin + 4..data_chunk_begin + 8]);
    
        println!("data_size: {}", data_size);
    
        let data_begin = data_chunk_begin + 8;
    
        let samples = data_size / fmt_num_channels as u32 / (fmt_bits_sample as u32 / 8);
    
        println!("all samples: {}", samples);
    
        println!("length of song: {} seconds", samples / fmt_sample_rate);
    
        let mut channel_data: Vec<i16> = Vec::with_capacity(data_size as usize);
    
        println!("{}", data_begin);
        println!("{}", data_size);
    
        for i in (0..data_size).step_by((fmt_bits_sample / 8) as usize) {
            let sample_value =
                LittleEndian::read_i16(&contents[data_begin + i as usize..2 + data_begin + i as usize]);
    
            channel_data.push(sample_value);
        }
        Ok((samples / fmt_sample_rate, channel_data))
    }

}
