mod csvs;
mod string_formatting;
mod string_types;
mod arrays;
mod vectors;
mod conditionals;

struct Person{
    name: String,
    age: i32,
    description: String
}

fn main() {
    // println!("Hello, world!");

    // let john = create_person("John".to_string(), 32, "aight".to_string());
    // println!("His name is {}", john.name);
    // println!("His age is {}", john.age);
    // println!("His is {}", john.description);

//    let path: String = "taxables.csv".to_string();
    // csvs::basic_csv_reader();
    // string_formatting::string_formatting_examples(); 

    // string_types::run()
    // arrays::run()
    // vectors::run()
    conditionals::run()
}

fn create_person(n: String, a: i32, d: String) -> Person {
    Person {
        name: n,
        age: a,
        description: d
    }
}

