// 🎯 Task 1: Conditional Logic
// Write a Rust program that takes two integers and prints whether they are equal, 
// the first is greater, or the second is greater using if-else statements.


pub fn task41() {
    if !cfg!(feature = "task41") {return;}

    let x = 3;
    let y = 3;

    if x == y {
        println!("x and y are equal");
    } else if x > y {
        println!("x is more than y");
    } else {
        println!("y is more than x");
    }
    
}
