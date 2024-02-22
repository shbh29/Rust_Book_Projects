use std::io::Read;

pub fn file_operation_with_match() {

    let file = std::fs::File::open("src/hello.txt");

    let mut file = match file {
        Ok(f) => f,
        Err(_e) => panic!("some error opening file. "),
    };

    let mut data = vec![];
    match file.read_to_end(&mut data) {
        Ok(size) => println!("content length: {}", size),
        Err(_e) => println!("Error reading file"),
    };

    println!("file content:\n {:?}", data.iter().map(|&b| b as char).collect::<Vec<char>>());
}

