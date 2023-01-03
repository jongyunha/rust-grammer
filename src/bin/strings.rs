struct Person {
    name: String,
    fav_color: String,
    age: i32,
}

fn print(data: &str) {
    println!("{}", data);
}

fn main() {
    let people = vec![
        Person {
            name: "George".to_owned(),
            fav_color: "green".to_owned(),
            age: 7,
        },
        Person {
            name: "Anna".to_owned(),
            fav_color: "purple".to_owned(),
            age: 9,
        },
        Person {
            name: "Katie".to_owned(),
            fav_color: "blue".to_owned(),
            age: 14,
        },
    ];

    for person in people {
        if person.age <= 10 {
            print(&person.name);
        }
    }
}
