use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

pub fn load_resources() -> io::Result<()> {
    // Read the file
    let f = File::open("resources.txt")?;
    let reader = BufReader::new(f);

    // Currently just printing out each line
    // Later update some vars
    for line in reader.lines() {
        println!("{}", line?);
    }

    Ok(())
}