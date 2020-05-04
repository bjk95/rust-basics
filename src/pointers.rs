pub fn run(){
    let vec1 = vec![1,3,5,9];
    let vec2 =  &vec1;

    println!("{:?}", (&vec1, vec2))
}