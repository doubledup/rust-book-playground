fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    let mut s2 = s1.clone();
    println!("Original copied value: '{}'", s2);
    change(&mut s2);
    println!("Changed to '{}'", s2);

    // let r1 = &mut s2;
    // let r2 = &mut s2;
    // println!("{}, {}", r1, r2);

    // let reference_to_nothing = dangle();
    let new_string = no_dangle();
    println!("Newly created string: {}", new_string);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) -> () {
    s.push_str(", world");
}

// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// }

fn no_dangle() -> String{
    let s = String::from("hello");
    s
}
