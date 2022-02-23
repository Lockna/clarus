use std::fs::File;
use std::io::Read;
use std::path::Path;

pub fn read_file(path: &Path) -> Result<Vec<u8>, std::io::Error> {
    let mut file = File::open(path)?;
    let mut contents = Vec::new();

    file.read_to_end(&mut contents).unwrap();

    Ok(contents)
}