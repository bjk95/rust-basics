
use std::fs::File;
use std::fs;
use std::io;

pub fn run () {

    #[allow(dead_code)]
    fn explicit_panic(){
     panic!("OH NOOOO BEECH")
    }

    #[allow(dead_code)]
    fn out_of_range(){
        let vec = vec![1,2,3];

        vec[99];
    }

    #[allow(dead_code)]
    fn open_file(){

        let f = File::open("./data/file.txt");
        let res = match f {
            Ok(file) => file,
            Err(error) => panic!("{:?}", error),
        };

        println!("{:?}", res)
    }

    fn read_file_to_string() -> Result<String, io::Error> {
        fs::read_to_string("./data/file.txt")        
    }

    let result: Result<String, io::Error> = read_file_to_string();
    match result{
        Ok(f) => println!("{:?}", f),
        Err(e) => panic!(e)
    }
}