pub fn day_001() {
if !cfg!(feature = "day_001") {return;}
// ------------------------------------------------------------
println!("---- Task 1 ----");

let x = 5;

#[allow(unused_assignments)]
{
let mut y = 10.5; 
y = 15.5;

println!("{}, {}", x, y);
}

// ------------------------------------------------------------
println!("---- Task 2 ----");

const MAX_USER: u32 = 100;
#[allow(unused_variables)]
let z = 2;
let z = 2 * 5;
println!("{}, {}", MAX_USER, z);

// ------------------------------------------------------------
println!("---- Task 3 ----");

let person: (i32, &str, f64) = (25, "Alice", 5.6);
let (age, name, height) = person;
println!("{}, {}, {}", age, name, height);

let arr: [i32; 5] = [1,2,3,4,5];
println!("{}, {}", arr[0], arr[4]);

// ------------------------------------------------------------
// challenge_day_continue "🎯➡️"
println!("---- Task 4 ----");
#[allow(unused_variables)]
let x = 5;
let x = 5 * 2;
println!("{}", x);
let x: String = x.to_string();
println!("{}", x);

// ------------------------------------------------------------
println!("---- Task 5 ----");

let mut count: i32 = 0;

for _ in 1..=10 {
    count += 1;
} 
println!("final value: {}", count);


// ------------------------------------------------------------
println!("---- Task 6 ----");

let sum = sum(2.2,2.2);

println!("{}", sum)

}

fn sum(a: f64, b: f64) -> f64 {
    a + b
}