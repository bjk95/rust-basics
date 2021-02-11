/// Adder function
/// 
/// should add together 2 numbers
/// 
/// # Examples
///  ```
/// use rust_basics::the_book::unit_test::add;
/// let added = add(5,2);
/// assert_eq!(added, 7)
/// ```

pub fn add(a: i32, b: i32) -> i32 {a + b}


/// Divide function
/// 
/// divides 2 integers
/// 
/// # Examples
/// ```
/// use rust_basics::the_book::unit_test::divide;
/// let divided = divide(4,2);
/// assert_eq!(divided, 2)
/// ```
/// 
/// # Panics
/// Cannot divide by 0
/// ```rust,should_panic
/// use rust_basics::the_book::unit_test::divide;
/// divide(4,0);
/// ``` 
/// 


pub fn divide(a: i32, b: i32) -> i32{
    if b == 0 {
        panic!("Divide-by-zero error");
    }
    a / b
}


pub fn run(){
    let num_string = add(5,3).to_string();

    println!("{}", num_string);
}
