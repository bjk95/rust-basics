pub fn run(){
    let mut vector: Vec<i32> = vec![1,2,3,4,5];


    println!("{:?}", &vector[0..4]);

    for num in vector.iter_mut(){
        println!("{:?}", *num*2);
    }


}