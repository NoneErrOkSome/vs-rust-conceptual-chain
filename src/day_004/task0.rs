// 🎯 Task 2: FizzBuzz with Match Expression
// Implement the classic FizzBuzz problem using a match expression. 
// Iterate through numbers from 1 to 100, 
// printing "Fizz" if the number is divisible by 3, 
// "Buzz" if it's divisible by 5, 
// and "FizzBuzz" if it's divisible by both 3 and 5. 
// For other numbers, print the number itself.

pub fn task40() {
    if !cfg!(feature = "task40") {return;}

    for i in 1..=100 {
        match i {
            _ if i % 3 == 0 && i % 5 == 0 => print!("FizzBuzz "),
            _ if i % 3 == 0 => print!("Fizz "),
            _ if i % 5 == 0 => print!("Buzz "),
            _ => print!("{} ", i)
        }
    }
}
