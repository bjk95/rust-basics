pub fn run(){

    fn beer_response(age: i32, looks_over_25: bool){
        if age < 18 {println!("Bad luck kiddo")}
        else if age >= 18 && age < 21 && looks_over_25 == false {
            println!("You aiit in normal countries, in murica still fucked tho")
    
        } else if age < 21 && looks_over_25 == true {
            println!("you sneaky beech you alright")
        }
        else {println!("You chill fam")};
    }

    let little = (16, false);
    let ehh = (19, false);
    let looks_old = (19, true);
    let big_boii = (21, false);
    let old = (30, true);

    let folks: Vec<(i32, bool)> = vec![little, ehh, looks_old, big_boii, old];

    for bloke in folks.iter(){
        beer_response(bloke.0, bloke.1)
    };
}