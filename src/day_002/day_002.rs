use std::f64::consts::PI;

enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Square(f64),
}

fn area(shape: Shape) -> f64 {
    match shape {
        Shape::Circle(radius) => PI * radius * radius,
        Shape::Rectangle(width, height) => width * height,
        Shape::Square(width) => width * width, // Assuming square's sides are equal
    }
}

fn some_slice(slice: &[i32]) -> i32 {
    let mut result = 0;
    for &i in slice {
        result += i;
    }
    result
}

fn converter(metre: i32) -> i32 {
    metre / 1000
}

fn length(string: String) -> usize {
    string.len()
}

fn borrow_string(reference: &str) -> usize {
    reference.len()
}

pub fn day_002() {
    if !cfg!(feature = "day_002") {
        return;
    }

    //-------------------------------------------------------------------
    println!("==== Task 1: Immutable vs Mutable Variables ====");
    //-------------------------------------------------------------------
    let x = 1; // immutable
    println!("{}", x);

    let mut y = 2; // mutable
    println!("{}", y);
    y = 3;
    println!("{}", y);

    //-------------------------------------------------------------------
    println!("==== Task 2: Scalar and Compound Types ====");
    //-------------------------------------------------------------------
    let x = 100;
    let y = 20.3;
    let z = true;
    let char = 'A';
    println!("{}, {}, {}, {}", x, y, z, char);

    let tuple = (1, "hello", 3.3, 5);
    let (zero, one, two, three) = tuple;
    println!("{}, {}, {}", zero, one, two);

    let my_array: [i32; 4] = [1, 2, 3, 4];
    println!("{:?}", my_array);


    // {
    //     let mut z: i32 = 2;
    //     z = 3;
    //     println!("{}, {}, {}", x, y, z);
    // }

    println!("{}, {}, {}, {}", zero, one, two, three);

    //-------------------------------------------------------------------
    println!("==== Task 3: Using Type Aliases ====");
    //-------------------------------------------------------------------
    type Metre = i32; // Fixed typo in type alias
    let jump: Metre = 1000; // Fixed variable name typo

    println!("Jumping distance: {} km", converter(jump));

    //-------------------------------------------------------------------
    println!("==== Task 4: Slices and Copy Trait ====");
    //-------------------------------------------------------------------
    let my_array: [i32; 5] = [1, 2, 3, 4, 5];
    let copy_trait: [i32; 5] = my_array;
    println!("copy trait: {:?}", copy_trait);

    let copy_again = my_array;
    println!("copy again: {:?}", copy_again);

    let sum_slice = some_slice(&my_array[2..=4]); // Fixed variable name
    println!("some_slice: {:?}", sum_slice);

    let some_array = &my_array[2..=4];
    println!("slice from my_array: {:?}", some_array);

    //-------------------------------------------------------------------
    println!("==== Task 5: Pattern Matching with Enums ====");
    //-------------------------------------------------------------------
    let circle = Shape::Circle(10.3);
    println!("Circle area: {:.3}", area(circle));

    let rectangle = Shape::Rectangle(2.3, 3.4);
    println!("Rectangle area: {:.3}", area(rectangle));

    let square = Shape::Square(22.3); // Fixed dimensions for square
    println!("Square area: {:.3}", area(square));

    //-------------------------------------------------------------------
    println!("==== Task 6: Ownership and Borrowing ====");
    //-------------------------------------------------------------------
    let s = String::from("Hello");
    let len = length(s);
    println!("{}", len);

    let str = String::from("reference");
    let length = borrow_string(&str);
    println!("{}", length);
    println!("{}", str); // reuse

}