// Task 5: Looping with while and for
// Write two versions of a program that prints numbers from 1 to 10. 
// Use while loop for the first version and for loop for the second version.
pub fn task45() {
    if !cfg!(feature = "task45") {return;}


    let mut x = 1;

    while x <= 10 {
        print!("{}, ", x);
        x += 1;
    
    }
println!(" ");

    for i in 1..=10 {
        print!("{}, ", i)
    }

}
