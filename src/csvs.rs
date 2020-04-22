extern crate csv;

use std::error::Error;
use std::io;
use std::process;

pub fn basic_csv_reader(path: String) -> Result<(), Box<dyn Error>> {
    let mut reader: Reader<Stdin> = csv::Reader::from_reader(io::stdin());
    for line in reader.records(){
        let result = line?;
        println!("{:?}", result)
    }
    Ok(())
}

pub fn typed_csv_reader()