#[allow(dead_code)]
pub fn print_string_literal() {
    // string literal &str -> fixed size
    // `static -> lifetime fixed
    let course: &str = "Summer School of Solana";
    let lecture: &str = "Rust";
    let company: &'static str = "AckeeBlockchain";

    println!(
        "I do attend {} lecture on {} from {}",
        course, lecture, company
    );
}
