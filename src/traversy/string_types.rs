pub fn run() {
    let mut hello:  String = String::from("hello ");
    let primative_bye: &str = "Goodbye";

    hello.push_str("u a BEECH ");
    hello.push('\u{1F604}');

    for i in hello.split_whitespace(){
        println!("{:?}",i);
    }

    println!("{}", hello);

    println!("{:?}", hello.replace("BEECH", "cutie"));


    println!("{}", primative_bye);


}