pub enum CarType {
    Hatch,
    Sedan,
    SUV,
}

pub fn print_size(car: CarType) {
    match car {
        // match is similar to switch statements
        CarType::Hatch => {
            println!("Small sized car");
        }
        CarType::Sedan => {
            println!("Medium sized car");
        }
        CarType::SUV => {
            println!("Large sized car");
        }
    }
}

pub fn display() {
    print_size(CarType::Hatch);
    print_size(CarType::Sedan);
    print_size(CarType::SUV);
}
