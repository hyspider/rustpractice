fn main() {

    //let x = (let y = 3);
        /*will get error. because "The let y = 6 statement does not return a
        value, so there isn’t anything for x to bind to";*/

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
    println!("five func {}", five());
    println!("six func {}", six());

    let x = plus_one(5);
    println!("The value of x is: {}", x);
}

fn six() -> i32
{
    6
}

fn five() -> i32
{
    let ret = 5;
    return ret;
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
/* get error example
fn plus_one(x: i32) -> i32 {
    x + 1; <====
}
*/
