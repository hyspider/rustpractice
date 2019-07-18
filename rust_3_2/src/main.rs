fn main() {
    let val: u32 = 0o77;

    println!("val {:?}", val);

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("{}", tup.0);

    let tup = (500, 6.4, 1);
    println!("{}", tup.1);

    let (x, y, z) = tup;
    println!("{} {} {}", x, y, z);

    let a = [1, 2, 3, 4, 5];
    println!("{}", a[2]);

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{}", a[3]);

    let a = [3; 5];
    println!("{} {}", a[0], a[4]);
}

