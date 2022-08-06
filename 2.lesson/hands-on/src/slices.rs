pub fn slice() {
    let ints: [i32; 5] = [1, 2, 3, 4, 5];
    let slice1: &[i32] = &ints[0..2]; // slice from index 0 to index 2-1
    let slice2: &[i32] = &ints[1..]; // open range

    println!("ints {:?}", ints);
    println!("slice 1 {:?}", slice1);
    println!("slice 2 {:?}", slice2);
}
