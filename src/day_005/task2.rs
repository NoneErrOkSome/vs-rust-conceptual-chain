fn convert(s: &str) -> i32 {
    s.parse().unwrap()
}


pub fn task52() {
    if !cfg!(feature = "task52") {return;}
    let string = convert("30");
    println!("{}", string);
    
}