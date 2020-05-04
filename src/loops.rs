pub fn run(){

    // Infitite loop
    let mut inf_count = 0;
    loop{
        inf_count += 1;
        println!("Count: {}", inf_count);

        if inf_count >= 10000 { break };
    }

    let mut num = 0;
    while num <= 100 {
        println!("{}", num);

        if num % 3 == 0 && num % 5 == 0 {println!("fizsbuzz")}

        else if num % 3 == 0 {println!("fizz")}

        else if num % 5 == 0 {println!("buzz")}

        num += 1
    }

    for x in 0..100{
        println!("{}", x);
    }
}