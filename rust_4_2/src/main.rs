fn main() {
    run_correct();
    run_error();
}

fn run_correct() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // r1 and r2 are no longer used after this point
    //
    let r3 = &mut s; // no problem
    println!("{}", r3);
}

fn run_error() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{}, {}", r1, r2);

    let r3 = &mut s; // BIG PROBLEM
    r3.push_str(" world3");
    println!("{}", r3);

    let r4 = &mut s; // BIG PROBLEM
    r4.push_str("!");
    println!("{}", r4);

    //println!("{}, {}, and {}", r1, r2, r3);
    println!("run error done {}", s);
}
