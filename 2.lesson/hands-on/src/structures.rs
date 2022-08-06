struct Person {
    first_name: String,
    last_name: String,
}

// implementation better way to initialise
impl Person {
    fn new(firstname: &str, lastname: &str) -> Person {
        Person {
            first_name: firstname.to_string(),
            last_name: lastname.to_string(),
        }
    }
}

pub fn person() {
    let p = Person::new("Elon", "Musk");
    println!("Welcome {} {}!", p.first_name, p.last_name);
}
