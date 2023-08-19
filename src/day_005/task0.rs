fn fibonacci(n: i32) -> i32 {
    if n == 0 {
        0
    } else if n == 0 {
        1
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}


pub fn task50() {
if !cfg!(feature = "task50") {return;}

    let n = 5;
    let result = fibonacci(n);
    println!("{}", result);
    
}