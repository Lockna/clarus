use clarus_utils::errors::WaveError;
use clarus_utils::decoder::Decoder;
use clarus_utils::reader::Reader;
use std::path::Path;
use std::time::Instant;

const PCM_FORMAT: u16 = 0x0001;
const IEEE_FLOAT: u16 = 0x0003;
const EXTENSIBLE_FORMAT: u16 = 0xFFFE;

pub struct WavDecoder {

    pub reader: Reader,
    pub format: u16,
    pub channels: u16,
    pub sample_rate: u32,
    pub bitdepth: u16,
    pub track_length: u32

}

impl WavDecoder {

    pub fn new(path: &Path) -> Self {
        WavDecoder {
            reader: Reader::new(path),
            format: 0,
            channels: 0,
            sample_rate: 0,
            bitdepth: 0,
            track_length: 0
        }
    }

}

impl Decoder for WavDecoder {
    fn decode(&mut self) -> Result<Vec<f32>, WaveError> {
        // Files from qobuz use id3v2
    
        let riff_str = self.reader.read_str();
    
        if riff_str != "RIFF" {
            return Err(WaveError::RIFFStringNotFound);
        }
    
        println!("{}", riff_str);
    
        let chunk_size = self.reader.read_u32_le();
    
        if chunk_size != self.reader.size() as u32 - 8 {
           return Err(WaveError::InvalidFileChunkSize);
        }
    
        println!("{}", self.reader.size());

        println!("{}", chunk_size);
    
        let wave_str = self.reader.read_str();
    
        if wave_str != "WAVE" {
           return Err(WaveError::WavStringNotFound);
        }
    
        println!("{}", wave_str);
    
        self.reader.seek_to_pattern(b"fmt ");

        let fmt_str = self.reader.read_str();
    
        println!("{}", fmt_str);

        if fmt_str != "fmt " {
            return Err(WaveError::FmtStringNotFound);
        }
    
        let fmt_size = self.reader.read_u32_le();
    
        println!("fmt_size: {}", fmt_size);

        if ![16,18,40].contains(&fmt_size) {
            return Err(WaveError::InvalidFormatSize);
        }
    
        let fmt_format_code = self.reader.read_u16_le();
    
        println!("fmt_format_code: {}", fmt_format_code);

        self.format = fmt_format_code;

        if !(self.format == PCM_FORMAT || self.format == EXTENSIBLE_FORMAT || self.format == IEEE_FLOAT) {
            return Err(WaveError::UnsupportedFormat);
        }
    
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

        if ![8,16,24,32,64].contains(&self.bitdepth) {
            return Err(WaveError::UnsupportedBitDepth)
        }

        if fmt_byte_rate != fmt_sample_rate * fmt_num_channels as u32 * fmt_bits_sample as u32/8 {
            return Err(WaveError::InvalidByteRate);
        }
    
        if fmt_block_align != fmt_num_channels * fmt_bits_sample/8 {
            return Err(WaveError::InvalidBlockAlign);
        }

        // TODO: rewrite seek_to_chunk or find better option
        self.reader.seek_to_chunk(b"data")?;
    
        let data_str = self.reader.read_str();
    
        println!("{}", data_str);
    
        let data_size = self.reader.read_u32_le();
    
        println!("data_size: {}", data_size);
    
        let samples = data_size / fmt_num_channels as u32 / (fmt_bits_sample as u32 / 8);
    
        println!("all samples: {}", samples);

        println!("{}", data_size);

        if data_size as u64 != (samples as u64 * fmt_num_channels as u64 * fmt_bits_sample as u64 / 8) as u64 {
           return Err(WaveError::InvalidDataSize);
        }
    
        self.track_length = samples / fmt_sample_rate as u32;

        println!("length of song: {} seconds", samples / fmt_sample_rate);
    
        let mut channel_data: Vec<f32> = Vec::with_capacity(data_size as usize / 2 as usize);

        println!("{}", data_size);
    
        let now = Instant::now();

        if self.format == PCM_FORMAT {

            channel_data.extend(
                (0..data_size)
                    .step_by((fmt_bits_sample / 8) as usize)
                    .map(|_| match self.bitdepth {
                        8 => self.reader.read_u8() as f32 / u8::MAX as f32,
                        16 => self.reader.read_i16_le() as f32 / i16::MAX as f32,
                        24 => self.reader.read_i24_le() as f32 / i32::MAX as f32,
                        32 => self.reader.read_i32_le() as f32 / i32::MAX as f32,
                        _ => unreachable!(),
                    }),
            )
        } else if self.format == IEEE_FLOAT {
            
            channel_data.extend(
                (0..data_size)
                    .step_by((fmt_bits_sample/8) as usize)
                    .map(|_| match self.bitdepth {
                        32 => self.reader.read_f32_le(),
                        // FIXME: Make 64 conversion lossless
                        64 => self.reader.read_f64_le() as f32,
                        _ => unreachable!(),
                    }),
            )

        } else if self.format == EXTENSIBLE_FORMAT {
            // TODO: Get an extensible format test file and implement decoding
            unimplemented!();
        }

        println!("{:?}", now.elapsed());

        println!("{}", channel_data.len());

        Ok(channel_data)

    }
}
