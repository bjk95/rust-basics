pub fn run() {
    let maybe_5: Option<u64> = Some(5);

    if let Some(x) = maybe_5 {
        println!("{}", x)
    }
}