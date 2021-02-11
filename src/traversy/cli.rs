use std::env;

pub fn run(){
    let args: Vec<String> = env::args().collect();

    let name = args[1].clone();

    print!("{:?}", args);

    print!("My name is {}", name)
}