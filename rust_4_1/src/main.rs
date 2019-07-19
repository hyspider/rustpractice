fn main() {
    let s = String::from("hello");
    println!("s is {}", s);

    let mut s = String::from("hello");
    s.push_str(", world");
    println!("s is {}", s);
    test_memcpy();
}

fn test_memcpy() {
    {
        //shallow copy example
        let s1 = String::from("hello");
        let s2 = s1; //shallow copy, s1 done here
        //println!("str1 {}", s1); <==== will get error, because s1 is done
        println!("str2 {}", s2);
    }


    {
        //deep copy example
        let s1 = String::from("hello");
        let s2 = s1.clone();
        println!("dc str1 {}", s1);
        println!("dc str2 {}", s2);
    }
}
