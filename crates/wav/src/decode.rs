use byteorder::LittleEndian;
use byteorder::ByteOrder;
use std::str;

pub fn decode(contents: Vec<u8>) {
        
        let riff_str = str::from_utf8(&contents[0..4]).unwrap();

        println!("{}", riff_str);

        let chunk_size = LittleEndian::read_u32(&contents[4..8]);

        println!("{}", contents.len());

        println!("{}", chunk_size);

        let wave_str = str::from_utf8(&contents[8..12]).unwrap();

        println!("{}", wave_str);

        let fmt_chunk_id = str::from_utf8(&contents[12..16]).unwrap();

        println!("fmt_chunk_id:{}", fmt_chunk_id);

        let fmt_chunk_size = LittleEndian::read_u32(&contents[16..20]);

        println!("{}", fmt_chunk_size);

        let format_code = LittleEndian::read_u16(&contents[20..22]);

        println!("{}", format_code);

        let number_of_channels = LittleEndian::read_u16(&contents[22..24]);

        println!("{}", number_of_channels);

        let sample_rate = LittleEndian::read_u32(&contents[24..28]);

        println!("{}", sample_rate);
}