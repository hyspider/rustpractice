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
    /*
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM

    println!("{}, {}, and {}", r1, r2, r3);
    */
    println!("run error done");
}
