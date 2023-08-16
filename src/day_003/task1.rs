// Task 1: Simple Function
fn sum(a: i32, b: i32) -> i32 {
    a + b
}

pub fn task31() {
    if !cfg!(feature = "task31") {return;}

let ans = sum(1, 2);
println!("{}", ans);

}

