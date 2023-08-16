
// Task 5: Closures
// Define a closure that calculates the factorial of a number 
// and use it within a function to calculate the factorial of 5.




pub fn task35() {
    if !cfg!(feature = "task35") {return;}


    let factorial = |n: u32| -> u32 { (1..=n).product() };
    let result = factorial(5);
    println!("The factorial of 5 is {}", result);

}