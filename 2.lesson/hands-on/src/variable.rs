#[allow(dead_code)]
pub fn print_string_obj() {
    // variables are immutable by default
    // unless mut keyword is used
    let mut txt_fees = 25_000;
    println!("Txt fees is {} ", txt_fees);
    txt_fees = 35_000;
    println!("Txt fees changed to {}", txt_fees);
}
