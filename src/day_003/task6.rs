// Task 6: Error Handling
// Create a function that takes two integers and divides them. 
// Use the Result type to handle the case when the second integer is zero, 
// returning an error message in that case.

fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("Cannot divide with zero".to_string())
    } else {
        Ok(a / b)
    }
}

pub fn task36() {
    if !cfg!(feature = "task36") {return;}

    // error handling
    let divided = divide(2, 0);
    println!("{:?}", divided);

}
