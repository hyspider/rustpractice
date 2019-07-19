fn main() {
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };

    println!("The value of number is: {}", number);
	run_loop();
    run_while();
    run_for();
    run_for_rev();
}


fn run_loop() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
}

fn run_while()
{
    let mut val = 0;
    while val != 10
    {
        val += 1;
        if val == 10
        {
            break;
        }
    }
    println!("while val {}", val);
}

fn run_for() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("run_for the value is: {}", element);
    }
}

fn run_for_rev() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
