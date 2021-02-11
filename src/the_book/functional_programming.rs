pub fn run() {

    // Vector of numbers
    let num_array = vec![1,4,5,6,7,2,10];

    // Doubling it 
    let doubled: Vec<u64> = num_array.iter().map( |x| x*2).collect();

    println!("Mapping over vector");
    println!("{:?}", doubled);

    // Summing 
    let summed: u64 = doubled.iter().sum();
    println!("\nSum of vector");
    println!("{:?}", summed);


    // filtering 
    let threshold: u64 = 10;
    let filtered: Vec<u64> = doubled.into_iter()
        .filter( |x| x > &&threshold)
        .map( |x| x *2)
        .collect();

    println!("\nFiltering vector");
    println!("{:?}", filtered);

    fn stringify(num: &u64) -> String {
        let number_string: &str = &num.to_string();
        String::from(format!("This number is {}", number_string))
    }

    let stringified: Vec<String> = num_array.iter().map(stringify).collect();
    println!("{:?}", stringified);

    let summed: u64 = filtered.iter().fold(0, | acc, x| acc + x);

    println!("\nSum of filtered is {}", summed)

 





}