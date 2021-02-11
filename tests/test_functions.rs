// extern crate rust_basics;

// #[cfg(test)]    
// mod test {
 
//      /// We need to use "use" declaration, because any other module is unaware of our "functions" module by default.
//      use rust_basics::the_book::unit_test::add;
//      /// Add
//      #[test]
//      fn test_add_positive_result() {
//         assert_eq!(2, add(1, 1));
//     }

//     #[test]
//     fn test_add_negative_result() {
//         assert_eq!(0, add(-1, 1));
//     }

//     #[test]
//     #[should_panic]    /// This means that we expect this test to fail (raise panic), so it will pass. 
//     fn test_add_panic() {
//         assert_eq!(3, add(1, 1));
//     }

// }