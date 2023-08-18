fn fibonacci(n: u32) -> u32 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}

pub fn task46() {
    if !cfg!(feature = "task46") {return;}

    let n = 10;
    println!("Fibonacci {} = {}", n, fibonacci(n));

}