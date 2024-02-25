use std::{fs, io};


pub struct Config {
    pub _query: String,
    pub filepath: String,
}

impl Config {
    pub fn build(vec: &Vec<String>) -> Result<Config, &'static str> {
        if vec.len() < 3 {
            return Err("not enough arguments!");
        }
        let query = vec[1].clone();
        let filepath = vec[2].clone();
    
        Ok(Config {_query: query, filepath: filepath})
    }
}

pub fn run(config: &Config) -> Result<(), io::Error>{
    let content = fs::read_to_string(&config.filepath)?; 
    println!("Content of file: \n {}", content);

    Ok(())
}