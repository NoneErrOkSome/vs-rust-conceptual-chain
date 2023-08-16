enum Shapes {
    Rectangle(f64, f64),
    Square(f64),
}

fn area(shape: Shapes) -> f64 {
    match shape {
        Shapes::Rectangle(width, height) => width * height,
        Shapes::Square(width) => width * width, 
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

pub fn day_003() {

    if !cfg!(feature = "day_003") {return;}

    let answer = sum(1, 2);
    println!("fn sum: {}", answer);


    let odd_even = odd_even(4);
    println!("odd_even: {}", odd_even);

    let cal_area = area(Shapes::Rectangle(3.3, 3.3));
    println!("enum shapes: {:.2}", cal_area);
    let square_area = area(Shapes::Square(5.3));
    println!("enum shapes: {:.2}", square_area);

    let pointer_multiply: fn(a: i32) -> i32 = mutiply;
    let result = pointer_multiply(3);
    println!("function pointer multiply: {}", result);


    let factorial = |n: u32| -> u32 {
        (1..=n).product()
        };
    let n = 5;
    println!("The factorial of {} is {}", n, factorial(n)); 

    
}