use clarus_utils::errors::Error;
use crate::read::WavReader;
use std::path::Path;
use std::time::Instant;

pub struct WavDecoder {

    pub reader: WavReader,
    pub format: u16,
    pub channels: u16,
    pub sample_rate: u32,
    pub bitdepth: u16
    // samples: decoded data

}

impl WavDecoder {

    pub fn new(path: &Path) -> Self {
        WavDecoder {
            reader: WavReader::new(path),
            format: 0,
            channels: 0,
            sample_rate: 0,
            bitdepth: 0
        }
    }

    // Maybe put the sampled data into the struct and just return empty result or error
    pub fn decode(&mut self) -> Result<(u32, Vec<i16>), Error> {
        // Files from qobuz use id3v2

        //let fmt_start = pattern::find_signature_index(&contents, b"fmt ").unwrap();
    
        let riff_str = self.reader.read_str();
    
        if riff_str != "RIFF" {
            return Err(Error::InvalidWAVFile);
        }
    
        println!("{}", riff_str);
    
        let chunk_size = self.reader.read_u32_le();
    
        if chunk_size != self.reader.size() as u32 - 8 {
           return Err(Error::InvalidWAVFile);
        }
    
        println!("{}", self.reader.size());

        println!("{}", chunk_size);
    
        let wave_str = self.reader.read_str();
    
        if wave_str != "WAVE" {
           return Err(Error::InvalidWAVFile);
        }
    
        println!("{}", wave_str);
    
        self.reader.seek_to_pattern(b"fmt ");

        let fmt_str = self.reader.read_str();
    
        println!("{}", fmt_str);
    
        let fmt_size = self.reader.read_u32_le();
    
        println!("fmt_size: {}", fmt_size);
    
        let fmt_format_code = self.reader.read_u16_le();
    
        println!("fmt_format_code: {}", fmt_format_code);

        self.format = fmt_format_code;
    
        let fmt_num_channels = self.reader.read_u16_le();
    
        println!("fmt_num_channels: {}", fmt_num_channels);

        self.channels = fmt_num_channels;
    
        let fmt_sample_rate = self.reader.read_u32_le();
    
        println!("fmt_sample_rate: {}", fmt_sample_rate);

        self.sample_rate = fmt_sample_rate;
    
        let fmt_byte_rate = self.reader.read_u32_le();
    
        println!("fmt_byte_rate: {}", fmt_byte_rate);
    
        let fmt_block_align = self.reader.read_u16_le();
    
        println!("fmt_block_align: {}", fmt_block_align);
    
        let fmt_bits_sample = self.reader.read_u16_le();
    
        println!("fmt_bits_sample: {}", fmt_bits_sample);

        self.bitdepth = fmt_bits_sample;
    
        self.reader.seek_to_pattern(b"data");
    
        let data_str = self.reader.read_str();
    
        println!("{}", data_str);
    
        let data_size = self.reader.read_u32_le();
    
        println!("data_size: {}", data_size);
    
        let samples = data_size / fmt_num_channels as u32 / (fmt_bits_sample as u32 / 8);
    
        println!("all samples: {}", samples);
    
        println!("length of song: {} seconds", samples / fmt_sample_rate);
    
        let mut channel_data: Vec<i16> = Vec::with_capacity(data_size as usize / 2 as usize);

        println!("{}", data_size);
    
        let now = Instant::now();

        for _ in (0..data_size).step_by((fmt_bits_sample / 8) as usize) {
            channel_data.push(self.reader.read_i16_le());
        }

        println!("{:?}", now.elapsed());

        println!("{}", channel_data.len());

        Ok((samples / fmt_sample_rate, channel_data))

    }

}
