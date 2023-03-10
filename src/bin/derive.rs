#[derive(Debug, Clone, Copy)]
enum Position {
    Manager,
    Supervisor,
    Worker,
}

#[derive(Debug, Clone, Copy)]
struct Employee {
    position: Position,
    work_hours: u8,
}

fn print_employee(employee: Employee) {
    println!("{:?}", employee);
}

fn main() {
    let me = Employee {
        position: Position::Manager,
        work_hours: 40,
    };
    print_employee(me);
    print_employee(me);
}
