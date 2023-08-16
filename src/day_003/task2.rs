// Task 2: Function with Conditional Logic
// Create a function that takes an integer and returns whether it's odd or even. 
// Call this function and print the result.

fn odd_even(a: i32) -> String {
    if a % 2 == 0 {
        format!("even number")
    } else {
        format!("odd number")
    }
}

pub fn task32() {
    if !cfg!(feature = "task32") {return;}

    
    let odd_even = odd_even(4);
    println!("odd_even: {}", odd_even);

}