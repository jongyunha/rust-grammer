enum Discount {
    Percentage(i32),
    Flat(i32),
}

struct Product {
    event: String,
    price: i32,
}

fn main() {
    let n = 3;
    match n {
        3 => println!("three"),
        other => println!("number: {:?}", other),
    }

    let flat = Discount::Flat(2);
    match flat {
        Discount::Flat(2) => println!("flat: 2"),
        Discount::Flat(amount) => println!("flat: {:?}", amount),
        _ => (),
    }

    let concert = Product {
        event: "concert".to_owned(),
        price: 100,
    };

    match concert {
        Product { price: 100, .. } => println!("price: 100"),
        Product { price, .. } => println!("price: {}", price),
    }
}
