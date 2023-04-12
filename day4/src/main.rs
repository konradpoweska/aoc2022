use std::env;

use day4::run;

fn main() -> Result<(), &'static str> {
    let filename = env::args().nth(1).ok_or("No filename provided.")?;
    run(&filename)
}
