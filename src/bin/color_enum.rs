enum Color {
    Red,
    Yello,
    Green,
    Blue
}

fn print_color(my_color: Color) {
    match my_color {
        Color::Red => print!("RED"),
        Color::Yello => print!("YELLO"),
        Color::Green => print!("GREEN"),
        Color::Blue => print!("BLUE"),
    }
}

fn main() {
    print_color(Color::Blue)
}