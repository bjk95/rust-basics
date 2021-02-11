pub fn run(){


    // Cloning
    let str1: String = String::from("Brad");

    fn say_hello(s: String) -> String {
        String::from("Hello, ") + &s
    };

    let hello_brad = say_hello(str1.clone());
    println!("{}", hello_brad);
    println!("{}", str1);
 
    // References
    let brad: String = String::from("Brad");

    fn is_beech(s: &str) -> String {
        String::from("Guess what, U A BEECH, ") + s 
    }

    let brad_beech = is_beech(&brad);

    println!("{}", brad_beech);
    println!("{}", brad);

    fn get_first_word(s: &str) -> &str {
        for (i, &ch) in s.as_bytes().iter().enumerate() {
            if ch == b' ' {
                return &s[..i]
            }
        }
        &s
    };

    let first_word = get_first_word(&brad_beech);

    println!("{}", first_word);





    #[allow(dead_code)]
    fn traversy(){
        let lit1 = 5;
        let lit2 = 10;

        println!("{:?}", (lit1, lit2));
        println!("{:?}", (lit1 + lit2 ));

        fn add_and_double(n1: i32, n2: i32) -> i32 {
            (n1 + n2) * 2
        }

        let doubled = add_and_double(lit1, lit2);

        println!("{}", doubled);
        println!("{:?}", (lit1, lit2));

        let a_vector = vec![1,4,8,9,23];
        println!("{:?}", a_vector);

        fn doubler (n: &i32) -> i32 {
            n*2
        }

        let x = String::from("hello");
        let y = x.clone() + &String::from(" world");

        

        println!("{}",x);
        println!("{}",y);
    }
}
