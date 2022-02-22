pub use std::io;

pub fn find_signature_index(data: &[u8], pattern: &[u8]) -> io::Result<usize> {
        data.windows(pattern.len()).position(|window| {
            pattern
                .iter()
                .zip(window.iter())
                .all(|(pattern_byte, mem_byte)| *pattern_byte == *mem_byte)
        }).ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::UnexpectedEof,
                    "failed to find signature pattern in memory",
                )
        })
}