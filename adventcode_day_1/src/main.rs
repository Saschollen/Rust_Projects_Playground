use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}



fn main() {
    let lines = lines_from_file("C:/Users/Owner/Rust_Projects/adventcode_day_1/input.txt").expect("Could not load lines");
    for line in lines {
        println!("{:?}", line);
    }
}