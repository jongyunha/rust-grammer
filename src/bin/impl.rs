struct Temperature {
    degrees_f: f64,
}

impl Temperature {
    fn freezing() -> Temperature {
        Temperature { degrees_f: 32.0 }
    }

    fn show_temp(&self) {
        println!("The temperature is {} degrees F", self.degrees_f);
    }
}

fn main() {
    let hot = Temperature { degrees_f: 90.0 };
    hot.show_temp();
    let freeze = Temperature::freezing();
    freeze.show_temp();
}
