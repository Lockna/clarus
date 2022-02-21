pub use std::io;
pub use crate::errors;

pub fn find_signature_ptr(data: &Vec<u8>, pattern: &str) -> io::Result<*const u8> {
        let pattern_elements: Vec<_> = pattern
            .split(' ')
            .map(|e| u8::from_str_radix(e, 16).ok())
            .collect();

        data.windows(pattern_elements.len())
            .find(|window| {
                pattern_elements
                    .iter()
                    .zip(window.iter())
                    .all(|(pattern_byte, mem_byte)| {
                        pattern_byte.map_or(false, |pred| pred == *mem_byte)
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
pub fn find_signature_index(data: &Vec<u8>, pattern: &str) -> Result<usize, errors::Error> {
        
        let ptr = find_signature_ptr(&data, &pattern);

        let index = ptr.unwrap() as usize - data.as_ptr() as usize;

        Ok(index)
}