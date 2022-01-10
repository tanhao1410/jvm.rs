use std::fs::File;
use std::io::Read;
use std::io::BufReader;

pub fn read_file(file: &File) -> Result<Vec<u8>,std::io::Error> {
    let mut buf_reader = BufReader::new(file);
    let mut res = vec![];
    buf_reader.read_to_end(&mut res)?;
    Ok(res)
}