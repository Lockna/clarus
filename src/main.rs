use std::path::Path;
use std::fs::File;
use std::io::Read;

fn main() {
    
    let path = Path::new("./test_files/Rammstein - Deutschland.wav");

    let file_contents = read_file(path).unwrap();

    clarus_wav::decode::decode(file_contents);

}

fn read_file(path: &Path) -> Result<Vec<u8>, std::io::Error> {
    let mut file = File::open(path)?;
    let mut contents = Vec::new();

    file.read_to_end(&mut contents).unwrap();

    Ok(contents)
}
