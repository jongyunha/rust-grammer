enum Access {
    Full,
}

fn one_two_three() -> (i32, i32, i32) {
    (1, 2, 3)
}

fn main() {
    let numbers = one_two_three();
    let (x, y, z) = one_two_three();

    let (employee, access) = ("Jake", Access::Full);

    numbers.0;
    numbers.1;
    numbers.2;
}