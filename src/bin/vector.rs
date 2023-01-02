fn main() {
    let my_numbers = vec![1, 2, 3];
    let mut my_numbers2 = Vec::new();
    my_numbers2.push(1);
    my_numbers2.push(3);
    my_numbers2.push(2);
    my_numbers2.pop();
    my_numbers2.len();
    my_numbers2.len();

    let two = my_numbers2[1];
    println!("The second element is {}", two);
}
