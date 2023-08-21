fn longer<'a>(s1: &'a str, s2: &'a str) {

    if s1.len() > s2.len() {
        println!("s1 has {} letter, s2 has {} letter", s1.len(), s2.len());
    } else {
        println!("s2 has {} letter, s1 has {} letter", s2.len(), s1.len());
    }
}


pub fn task54() {
    if !cfg!(feature = "task54") {return;}

    let a = "haha";
    let b = "ha";
    longer(a, b);

}