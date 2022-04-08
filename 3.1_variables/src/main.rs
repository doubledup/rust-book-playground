const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    let mut x = 5;
    let y = 14;
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    x = 6;
    let y = 42;
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);

    println!("---");

    println!("Three hours contains {} seconds", THREE_HOURS_IN_SECONDS);

    println!("---");

    let z = 7;
    let z = z + 1;

    {
        let z = z * 2;
        println!("The value of z in the inner scope is: {}", z);
    }

    println!("The value of z is: {}", z);
}
