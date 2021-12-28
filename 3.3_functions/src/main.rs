fn main() {
     let z = plus_one(5);
     println!("The value of z is: {}", z);

    let x = five();
    println!("The value of x is: {}", x);

    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {}", y);

    // let x = (let y = 6);

    print_labeled_measurement(5, 'h');

    print_value(5);

    another_function();
}

fn plus_one(x: i32) -> i32 {
    return x + 1;
}

fn five() -> i32 {
    5
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}

fn print_value(x: i32) {
    println!("The value of x is: {}", x);
}

fn another_function() {
    println!("Another function.")
}
