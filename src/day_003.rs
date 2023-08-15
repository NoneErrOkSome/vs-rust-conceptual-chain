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
    println!("{}", answer);


    let odd_even = odd_even(4);
    println!("{}", odd_even);

    let cal_area = area(Shapes::Rectangle(3.3, 3.3));
    println!("{:.2}", cal_area);
    let square_area = area(Shapes::Square(5.3));
    println!("{:.2}", square_area);

  
}