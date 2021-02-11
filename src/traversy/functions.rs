pub fn run(){
    fn adder(n1: i32, n2: i32) -> i32 {
        n1 + n2
    }

    println!("{:?}", adder(5, 7));


    // Closures

    let multiple_nums = |n1: i32, n2: i32| n1 * n2;

    println!("{:?}", multiple_nums(5,6));

    let n3 = 10;
    let multiple_nums_with_outside_score = |n1: i32, n2: i32| n1 * n2 * n3;

    println!("{:?}", multiple_nums_with_outside_score(5,6));

    let vector = vec![1,3,5,7,11];
    
    let doubled = vector.iter().map(|x| x * 2);

    for num in doubled{println!("{}", num)};
}