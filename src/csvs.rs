use std::error::Error;
use std::io;
use std::io::Stdin;
use std::process;
use csv::Reader;


use serde::Deserialize;

// By default, struct field names are deserialized based on the position of
// a corresponding field in the CSV data's header record.
#[derive(Debug, Deserialize)]
struct Item{
    index: String,
    item: String,
    cost: String,
    tax: String,
    total: String,
    is_lit: String,
}

pub fn example() -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(io::stdin());
    for result in rdr.deserialize() {
        // Notice that we need to provide a type hint for automatic
        // deserialization.
        let record: Item = result?;
        println!("{:?}", record);
    }
    Ok(())
}



pub fn basic_csv_reader() -> Result<(), Box<dyn Error>> {
    let mut reader: Reader<Stdin> = csv::Reader::from_reader(io::stdin());
    for line in reader.records(){
        let result = line?;
        println!("{:?}", result)
    }
    Ok(())
}
//
//pub fn typed_csv_reader() -> Result<(), Box<Error>> {
//    let mut reader = csv::Reader::from_reader(io::stdin());
//    for line in reader.deserialize(){
//        let result: Item = line?;
//        println!("{:?}", result.item)
//    }
//    Ok(())
//}

