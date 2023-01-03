struct Customer {
    age: Option<i32>,
    email: String,
}

fn main() {
    let mark = Customer {
        age: Some(10),
        email: "popawaw@naver.com".to_owned(),
    };
    let jongyun = Customer {
        age: None,
        email: "popawaw@naver.com".to_owned(),
    };

    match jongyun.age {
        Some(age) => println!("age: {}", age),
        None => println!("no age"),
    }
}
