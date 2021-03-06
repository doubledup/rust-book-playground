fn main() {
    let s = "World";
    {
        let s = "Hello";
        println!("{}", s);
    }
    println!("{}", s);

    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);

    // let s1 = String::from("hello");
    // let s2 = s1;
    // println!("{}, world!", s1);

    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);

    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);

    let s = String::from("hello");
    let s = takes_ownership(s);
    println!("returned string: {}", s);
    let x = 5;
    makes_copy(x);
    println!("copied int: {}", x);
}

fn takes_ownership(some_string: String) -> String {
    println!("{}", some_string);
    some_string
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}
