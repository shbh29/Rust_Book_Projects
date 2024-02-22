use std::fs::File;
use std::io::Read;

pub fn unwrap_method_for_file_opening() {
    let mut file = File::open("src/hello.txt").unwrap();

    let mut data = Vec::new();
    file.read_to_end(&mut data).unwrap();

    println!("file data: {:?}", data.iter().map(|b| *b as char).collect::<String>().len());
}
