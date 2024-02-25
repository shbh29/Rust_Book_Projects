use std::{fs, io};


pub struct Config {
    pub query: String,
    pub filepath: String,
}

impl Config {
    pub fn build(vec: &Vec<String>) -> Result<Config, &'static str> {
        if vec.len() < 3 {
            return Err("not enough arguments!");
        }
        let query = vec[1].clone();
        let filepath = vec[2].clone();
    
        Ok(Config {query, filepath: filepath})
    }
}

pub fn run(config: &Config) -> Result<(), io::Error>{
    let content = fs::read_to_string(&config.filepath)?; 
    let found = search(&config.query, &content);
    
    for line in found {
        println!("{}", line);
    }

    Ok(())
}

fn search<'a>(query: &'a str, content: &'a str) -> Vec<&'a str> {
    let mut found = Vec::new();

    for line in content.lines() {
        if line.contains(query) {
            found.push(line);
        }
    }

    found
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn search_through_content() {
        let query = "duct";
        let content = "\
        Rust,
        Production server should have rust,
        I only believe and don't know the future.";

        assert_eq!(vec!["        Production server should have rust,"], search(&query, &content));
    }
}