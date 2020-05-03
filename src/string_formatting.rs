use std::error::Error;

pub fn string_formatting_examples() -> Result<(), Box<dyn Error>>{
    println!("{0} is from {1} and {0} likes to {2}", "Brad", "straya", "push pedals");


    let name = "Brad";
    let activity = "ride bikes";
    println!("{name} likes to {activity}", name=name, activity=activity);
    println!("{} likes to {}2", name, activity);

    // Placeholder traits
    println!("Binary {:b}, hex {:X}, oct {:o}", 70, 70,203);

    // PLaceholder for debug trait
    println!("{:?}", (7, false, "kh"));

 
    let unicode = '\u{1F604}';
    println!("when {} rides bikes he is {}", name, unicode);
    
    Ok(())
}