#[allow(dead_code)]
pub fn ownership() {
    // how rust handles memory -> ownership
    // each value in rust has a variable called its owner
    // only one owner at a time
    // when owwner goes out of scope, the value will be dropped
    let v = vec![1, 2, 3];
    let v2 = v;
    println!("{:?}", v2);
}
