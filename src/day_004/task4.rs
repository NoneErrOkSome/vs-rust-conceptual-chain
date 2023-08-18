// Task 4: Pattern Matching with Enums
// Define an enum representing traffic light colors (Red, Yellow, Green). 
// Write a function that takes a traffic light color and returns the amount of time (in seconds) 
// that the light stays on that color. Use pattern matching.
enum Colors {
    Red(i32),
    Yellow(i32),
    Green(i32),
}

fn light_color(colors: Colors) {
    match colors {
        Colors::Red(red_color) => println!("{} seconds", red_color),
        Colors::Yellow(yellow_color) => println!("{} seconds", yellow_color),
        Colors::Green(green_color) => println!("{} seconds", green_color),
    }

}




pub fn task44() {
    if !cfg!(feature = "task44") {return;}

light_color(Colors::Red(60));
light_color(Colors::Yellow(4));
light_color(Colors::Green(30));

}
