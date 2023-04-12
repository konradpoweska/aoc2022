use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn lines_from_stdin() -> impl Iterator<Item = String> {
    std::io::stdin().lines().map(|e| e.expect("Invalid line"))
}

pub fn lines_from_file(filename: &str) -> Result<impl Iterator<Item = String>, &'static str> {
    let file = File::open(&filename).or(Err("Couldn't open file."))?;
    Ok(BufReader::new(file).lines().map(Result::unwrap))
}
