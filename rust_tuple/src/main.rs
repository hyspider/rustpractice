fn main() {
    let tuple = (5, String::from("five"));

    // Here, tuple is _not_ moved, as the String was never moved, and u32 is Copy:
     let (x, _) = tuple;
    //
     // That means this works:
    println!("Tuple is: {:?}", tuple);
}
