

pub fn task50() {
if !cfg!(feature = "task50") {return;}


let mut n = 3;
    print!("{} ", n);
    while n != 1 {
        if n % 2 == 0 {
            n /= 2;
        } else {
            n = 3 * n + 1;
        }
        print!("{} ", n);
    }
    println!();


}