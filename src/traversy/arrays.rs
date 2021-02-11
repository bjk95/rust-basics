pub fn run() {
    let mut numbers: [i32; 6] = [1,2,3,4,5, 777];


    numbers.reverse();
    for num in numbers.iter(){
        println!("{:?}", num);
    }

    println!("{:?}", std::mem::size_of_val(&numbers));

    let slice: &[i32] = &numbers[0..3];
    println!("{:?}", slice);
}