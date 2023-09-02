fn longer<'a>(s1: &'a str, s2: &'a str) -> &'a str {

    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}


pub fn task54() {
    if !cfg!(feature = "task54") {return;}

    let a = "haha";
    let b = "ha";
    let result = longer(a, b);
    println!("{} ", result );

}