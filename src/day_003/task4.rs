// Task 4: Function Pointer
// Create a function that takes another function (e.g., doubling a number) 
// and an integer as arguments. 
// Use this higher-order function to apply the passed function to the integer.


fn mutiply(a: i32) -> i32 {
    a * a
}



pub fn task34() {
    if !cfg!(feature = "task34") {return;}

    let pointer_multiply: fn(a: i32) -> i32 = mutiply;
    let result = pointer_multiply(3);
    println!("function pointer multiply: {}", result);


    
}