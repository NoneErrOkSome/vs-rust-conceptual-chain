// Task 7: Recursion
// Write a recursive function to calculate the Fibonacci sequence up to the nth element.
// Call this function and print the result.

fn fibonacci(n: u32) -> u32 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}

pub fn task37() {
    if !cfg!(feature = "task37") {return;}

    let n = 10;
    println!("Fibonacci {} = {}", n, fibonacci(n));

}