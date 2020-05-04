// Traditional struct
struct Colour {
    red: u8,
    green: u8,
    blue: u8
}

// Tuple Struct
struct TColour (u8,u8,u8);

struct Customer {
    first_name: String,
    last_name: String,
    is_active: bool,
    age: u16
}

impl Customer {
    fn new(first_name: &str, last_name: &str, year_of_birth: u16) -> Customer {

        Customer{
            first_name: first_name.to_string(),
            last_name: String::from(last_name),
            is_active: true,
            age: 2020-year_of_birth 
        }
    }
    fn print_full_name(&self){
        println!("{}, {}", self.last_name, self.first_name)
    }

    fn whack_him(&mut self) {
        self.is_active = false;
    }
}

pub fn run(){
    let c = Colour{
        red: 255,
        green: 0,
        blue: 0
};

    println!("{:?}", (c.red, c.green, c.blue));

    
    let ct = TColour(0, 255, 0);
    println!("{:?}", (ct.0, ct.1, ct.2));

    let mut johnny = Customer::new("Johnny", "The Snitch", 1965);


    johnny.print_full_name();
    // println!("{:?}", (johnny.first_name, johnny.last_name, johnny.is_active, johnny.age));


    johnny.whack_him();

    println!("{:?}", (johnny.first_name, johnny.last_name, johnny.is_active, johnny.age));


    


}