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

pub fn day_003_1() {
    if !cfg!(feature = "day_003_1") {return;}

    let cal_area = area(Shapes::Rectangle(3.3, 3.3));
    println!("enum shapes: {:.2}", cal_area);
    let square_area = area(Shapes::Square(5.3));
    println!("enum shapes: {:.2}", square_area);
    
}