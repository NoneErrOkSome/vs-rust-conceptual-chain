fn divide(num: i32, divisor: i32) -> Result<i32,  &'static str> {
    if divisor != 0 {
        Ok(num / divisor)
    } else {
        Err("Cannot divide by zero")
    }
}

pub fn task51() {
    if !cfg!(feature = "task51") {return;}
    let divided = divide(10, 2);
    println!("{:?}", divided);

}