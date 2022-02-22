pub use std::io;
pub use crate::errors;

pub fn find_signature_ptr(data: &[u8], pattern: &[u8]) -> io::Result<*const u8> {
        data.windows(pattern.len())
            .find(|window| {
                pattern
                    .iter()
                    .zip(window.iter())
                    .all(|(pattern_byte, mem_byte)| {
                        *pattern_byte == *mem_byte
                    })
            })
            .map(|window| window.as_ptr())
            .ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::UnexpectedEof,
                    "failed to find signature pattern in memory",
                )
            })
}


// Dirty hack, find better solution for index finding
pub fn find_signature_index(data: &Vec<u8>, pattern: &[u8]) -> Result<usize, errors::Error> {
        
        let ptr = find_signature_ptr(&data, &pattern);

        let index = ptr.unwrap() as usize - data.as_ptr() as usize;

        Ok(index)
}