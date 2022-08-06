// option can eval smth or none

pub fn might_print(option: Option<&str>) {
    match option {
        Some(text) => println!("The argument contains the following value: '{}'", text),
        None => println!("The argument contains None."),
    }
}
pub fn contains_char(text: &str, target_c: char) -> Option<&str> {
    if text.chars().any(|ch: char| ch == target_c) {
        return Some(text);
    } else {
        return None;
    }
}

// Copy to main.rs
pub fn doSomething() {
    let something: Option<&str> = Some("Some string is inside");
    let nothing: Option<&str> = None;
    might_print(something);
    might_print(nothing);
}

pub fn checkForMatchingCharacters() {
    let a: Option<&str> = contains_char("Winter school of Solana", 'W');
    let q: Option<&str> = contains_char("Summer School of Solana", 'W');
    println!("{:?}", a);
    println!("{:?}", q);
}
