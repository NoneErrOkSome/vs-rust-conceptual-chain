
// 🎯 Task 7: Iterating Over a Vector with for Loop
// Create a vector of strings representing a list of tasks. 
// Iterate over the vector using a for loop and print each task.

pub fn task47() {
    if !cfg!(feature = "task47") {
        let vec: Vec<i32> = vec![1,2,3,4,5];
        for i in &vec {
            println!("{}", i);
        }

    }
}