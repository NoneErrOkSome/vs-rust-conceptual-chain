

fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("Cannot divide with zero".to_string())
    } else {
        Ok(a / b)
    }
}


fn sum(a: i32, b: i32) -> i32 {
    a + b
}

fn mutiply(a: i32) -> i32 {
    a * a
}

fn odd_even(a: i32) -> String {
    if a / 2 == 0 {
        format!("even number")
    } else {
        format!("odd number")
    }
}

pub fn task0() {

    if !cfg!(feature = "task30") {return;}

    let answer = sum(1, 2);
    println!("fn sum: {}", answer);


    let odd_even = odd_even(4);
    println!("odd_even: {}", odd_even);



// function pointer
    let pointer_multiply: fn(a: i32) -> i32 = mutiply;
    let result = pointer_multiply(3);
    println!("function pointer multiply: {}", result);

// closures
    let factorial = |n: u32| -> u32 {
        (1..=n).product()
        };
    let n = 5;
    println!("The factorial of {} is {}", n, factorial(n)); 

// error handling
    let divided = divide(2, 0);
    println!("{:?}", divided);

// Recursion
    let n = 5;
    println!("The factorial of {} is {}", n, factorial(n));
    
}

#[allow(dead_code)]
fn factorial(n: u64) -> u64 {
    if n == 0 {
        1 // Base case: if n is 0, return 1
    } else {
        n * factorial(n - 1) // Recursive case: multiply n by the factorial of (n-1)
    }
}
