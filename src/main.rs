
// mod traversy;
pub mod the_book;
// mod the_book::unit_test;
pub mod traversy;
use traversy::string_types;

// use the_book::unit_test::run;


fn main() {
    string_types::run();

    let number = 10;

    fn do_math(a: &i32, b: &i32) -> i32 {
        let c: i32 = a + b;
        c
    }


    let num1 = 1;
    let num2 = 2;
    let num = do_math(&num1, &num2);
    println!("{:?}",num);


    // let s1 = String::from("why hello");
    // let s2 = s1;

    // println!("{:?}",s1)



    // fn mut_num(num: &i32){
    //     num += 1
    // }

    let vec: Vec<i8> = vec![5,6,7,8];
    fn double_vec(v: &Vec<i8>) -> Vec<i8> {
        v.iter().map(|n| n* 2).collect()
    }

    println!("{:?}", double_vec(&vec));
    println!("{:?}", vec);
    let array = [1,2,3,4];

    fn double(a: [i8; 4]) -> Vec<i8>{
        let doubled = a.iter().map(|n| n *2);
        let collected: Vec<i8> = doubled.collect();
        collected
    };

    println!("{:?}", double(array));

    #[derive(Debug)]
    struct Person {
        name: String,
        age: i64
    }

    let mum = Person{name: String::from("Gena"), age: 175};
    println!("{:?}", mum);

    fn how_old(p: Person){
        if p.age > 100 {
            println!("She OLLLDDD")
        } else if p.age > 60 {
            println!("She getting there")
        } else {
            println!("Nah she aiigt")
        }
    }

    let not_pete = Person{name: String::from("Not Pete"), age: 75};
    let brad = Person{name: "B-rad".to_string(), age: 25};
    how_old(mum);
    how_old(not_pete);
    how_old(brad)
}

