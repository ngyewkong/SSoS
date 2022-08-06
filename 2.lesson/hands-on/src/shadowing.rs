#[allow(dead_code)]
pub fn print_string_obj() {
    // shadowing concept in rust
    // variable can be shadowed in the scope level
    let name = "AckeeBlockchain";
    println!("Hi from: {}", name); // name="AckeeBlockchain"
    let name = name.len();
    println!("name changed to ... {}", name); // name=15
}
