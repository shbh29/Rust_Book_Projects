use std::io::{self, Read};



pub fn read_file(mut data: &mut Vec<u8>) -> Result<usize, io::Error> {
    let size = std::fs::File::open("src/hello.txt")?.read_to_end(&mut data)?;
    Ok(size)
}