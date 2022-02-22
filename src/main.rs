use rodio::{buffer::SamplesBuffer, OutputStream};
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::thread;
use std::time::Duration;

fn main() {
    // Written by 0x6d70
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("Usage: clarus <filename>");
        return;
    }

    let path = Path::new(&args[1]);

    let file_contents = read_file(path).unwrap();

    let (song_length, channel_data) = clarus_wav::decode::decode(file_contents).unwrap();

    //match clarus_wav::decode::decode(file_contents) {
    //    Err(InvalidWAVFile) => println!("invalid"),
    //    Ok((song_length, channel_data)) =>
    //}

    let (_stream, stream_handle) = OutputStream::try_default().expect("Failed to open stream");

    let mut f = Vec::new();

    for i in (0..channel_data.len()).step_by(2) {
        // Figure out why divide by i16::MAX
        f.push(channel_data[i] as f32 / i16::MAX as f32);
    }

    stream_handle
        .play_raw(SamplesBuffer::new(1, 44100 as u32, f))
        .expect("Failed to play");

    thread::sleep(Duration::from_millis(song_length as u64 * 1000));
}

fn read_file(path: &Path) -> Result<Vec<u8>, std::io::Error> {
    let mut file = File::open(path)?;
    let mut contents = Vec::new();

    file.read_to_end(&mut contents).unwrap();

    Ok(contents)
}
