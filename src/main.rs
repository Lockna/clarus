use rodio::{buffer::SamplesBuffer, OutputStream};
use std::path::Path;
use std::thread;
use std::time::Duration;
use clarus_wav::decode::WavDecoder;

// ideas:
// maybe add trait which each decoder implements (functions like decode or something)

fn main() {
    // Written by 0x6d70
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("Usage: clarus <filename>");
        return;
    }

    let path = Path::new(&args[1]);

    let mut wav_decoder = WavDecoder::new(path);

    let decode_result = wav_decoder.decode();

    println!("after decode");

    if let Err(error) = decode_result {
        println!("{:?}", error);
        return;
    }

    let channel_data = decode_result.unwrap();

    let (_stream, stream_handle) = OutputStream::try_default().expect("Failed to open stream");

    //f.extend(channel_data.into_iter().map(|i| i as f32 / i16::MAX as f32));

    stream_handle
        .play_raw(SamplesBuffer::new(wav_decoder.channels, 44100 as u32, channel_data))
        .expect("Failed to play");

    
    thread::sleep(Duration::from_secs(wav_decoder.track_length as u64));
    
}
