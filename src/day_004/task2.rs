// 🎯 Task 2: FizzBuzz with Match Expression
// Implement the classic FizzBuzz problem using a match expression. 
// Iterate through numbers from 1 to 100, 
// printing "Fizz" if the number is divisible by 3, 
// "Buzz" if it's divisible by 5, 
// and "FizzBuzz" if it's divisible by both 3 and 5. 
// For other numbers, print the number itself.

pub fn task42() {
    if !cfg!(feature = "task42") {return;}

    for i in 1..=100 {
        match i {
            _ if i % 3 == 0 && i % 5 == 0 => println!("FizzBuzz"),
            _ if i % 3 == 0 => println!("Fizz"),
            _ if i % 5 == 0 => println!("Buzz"),
            _ => println!("{}", i),
        }
    }
    


    // for i in 0..=100 {
    //     if i % 3 == 0 {
    //         println!("{}, Buzz", i)
            
    //     } else if i % 5 == 0 {
    //         println!("{}, FizzBuzz", i)
    //     } else if i % 5 == 0 && i % 3 == 0 {
    //         println!("{}", i);
    //     }
    // }



}
