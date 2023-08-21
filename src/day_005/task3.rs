fn length(s: String) -> u32 {
    s.len().try_into().unwrap()
}

fn borrowed(str: &String) {
    println!("{}", str);
}

pub fn task53() {
    if !cfg!(feature = "task53") {return;}
    let str = String::from("Hello");
    let length = length(str);
    println!("{}", length);

    let borrow = "You've borrowed".to_string();
    borrowed(&borrow);
    
}