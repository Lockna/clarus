use clarus_utils::errors::InvalidWaveFile;
use crate::read::WavReader;
use std::path::Path;
use std::time::Instant;

pub struct WavDecoder {

    pub reader: WavReader,
    pub format: u16,
    pub channels: u16,
    pub sample_rate: u32,
    pub bitdepth: u16,
    pub track_length: u32

}

impl WavDecoder {

    pub fn new(path: &Path) -> Self {
        WavDecoder {
            reader: WavReader::new(path),
            format: 0,
            channels: 0,
            sample_rate: 0,
            bitdepth: 0,
            track_length: 0
        }
    }

    pub fn decode(&mut self) -> Result<Vec<i16>, InvalidWaveFile> {
        // Files from qobuz use id3v2
    
        let riff_str = self.reader.read_str();
    
        if riff_str != "RIFF" {
            return Err(InvalidWaveFile::RIFFStringNotFound);
        }
    
        println!("{}", riff_str);
    
        let chunk_size = self.reader.read_u32_le();
    
        if chunk_size != self.reader.size() as u32 - 8 {
           return Err(InvalidWaveFile::InvalidFileChunkSize);
        }
    
        println!("{}", self.reader.size());

        println!("{}", chunk_size);
    
        let wave_str = self.reader.read_str();
    
        if wave_str != "WAVE" {
           return Err(InvalidWaveFile::WavStringNotFound);
        }
    
        println!("{}", wave_str);
    
        self.reader.seek_to_pattern(b"fmt ");

        let fmt_str = self.reader.read_str();
    
        println!("{}", fmt_str);

        if fmt_str != "fmt " {
            return Err(InvalidWaveFile::FmtStringNotFound);
        }
    
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

        if fmt_byte_rate != fmt_sample_rate * fmt_num_channels as u32 * fmt_bits_sample as u32/8 {
            return Err(InvalidWaveFile::InvalidByteRate);
        }
    
        if fmt_block_align != fmt_num_channels * fmt_bits_sample/8 {
            return Err(InvalidWaveFile::InvalidBlockAlign);
        }

        // TODO: rewrite seek_to_chunk or find better option
        self.reader.seek_to_chunk(b"data");
    
        let data_str = self.reader.read_str();
    
        println!("{}", data_str);
    
        let data_size = self.reader.read_u32_le();
    
        println!("data_size: {}", data_size);
    
        let samples = data_size / fmt_num_channels as u32 / (fmt_bits_sample as u32 / 8);
    
        println!("all samples: {}", samples);

        println!("{}", data_size);

        if data_size as u64 != (samples as u64 * fmt_num_channels as u64 * fmt_bits_sample as u64 / 8) as u64 {
           return Err(InvalidWaveFile::InvalidDataSize);
        }
    
        self.track_length = samples / fmt_sample_rate as u32;

        println!("length of song: {} seconds", samples / fmt_sample_rate);
    
        let mut channel_data: Vec<i16> = Vec::with_capacity(data_size as usize / 2 as usize);

        println!("{}", data_size);
    
        let now = Instant::now();

        for _ in (0..data_size).step_by((fmt_bits_sample / 8) as usize) {
            channel_data.push(self.reader.read_i16_le());
        }

        println!("{:?}", now.elapsed());

        println!("{}", channel_data.len());

        Ok(channel_data)

    }

}
