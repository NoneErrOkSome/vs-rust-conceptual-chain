
pub fn task30() {
    if !cfg!(feature = "task30") {return;}

    let factorial = |n: i32| -> i32 { (1..=n).product() };
    let result = factorial(3);
    println!("{}", result);

}


