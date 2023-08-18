// Task 3: Using Nested Loops
// Create a program that prints a right-angled triangle using asterisks (*) with nested loops. 
// Allow the user to input the number of rows.


pub fn task43() {
    if !cfg!(feature = "task43") {return;}

    for i in 1..=10 {
        for y in 1..=1 {
            let s = "*".repeat(y*i);
            println!("{}", s);
        }
    }

    let rows = 10; 
    for i in 1..=rows {
        let s = "*".repeat(i);
        println!("{}", s);
    }
}
