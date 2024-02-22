pub mod with_question_mark;

pub fn handle_error() {
    let mut data = Vec::new();
    match with_question_mark::read_file(&mut data) {
        Ok(_) => println!("data in file: {}", data.iter().map(|b| *b as char).collect::<String>()),
        Err(e) => print!("Error reading file: {}", e),
    }
}