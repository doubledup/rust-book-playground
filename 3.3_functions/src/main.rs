fn another_function() {
    println!("Another function.")
}

fn main() {
    println!("Hello, world!");
    another_function();

    println!("---");

    print_value(5);

    println!("---");

    print_labeled_measurement(5, 'h');

    println!("---");

    // let x = (let y = 6);
    let _answer: u8 = 42;
    let answer_is_42 = matches!(42, _answer);
    println!("Is the answer 42? {}", if answer_is_42 { "Yes" } else { "No" });

    println!("---");

    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {}", y);

    println!("---");

    let x = five();
    println!("The value of x is: {}", x);

    let x = plus_one(x);
    println!("The value of x is now: {}", x);
}

fn print_value(x: u8) {
    println!("The value of x is: {}", x);
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}

fn five() -> i32 {
    // return can be an expression, not just a statement:
    return 5
}

fn plus_one(x: i32) -> i32 {
    return x + 1;
}
