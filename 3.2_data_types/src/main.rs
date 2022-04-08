fn main() {
    // integer overflow

    // Default panicking in debug mode, wrapping in release mode
    // let num: u8 = "255".trim().parse().expect("255 should be a number");
    // println!("{} + 1 = {}", num, num + 1);

    // let a = 2_i32.pow(31);
    // println!("a is {}", a);

    // println!("---");

    // Cannot combine signed & unsigned integers, even if they have the same precision.
    // println!("{}", 3_45i32 + 1_23u32);

    // Numbers out of range for the default type must be explicitly typed.
    // let b = 2147483648;

    let _x = 2.0;
    let _y: f32 = 3.0;

    let _sum = 5 + 10;
    let _difference = 95.5 - 4.3;
    let _product = 4 * 30;
    let _quotient = 56.7 / 32.2;
    let _floored = 2 / 3;
    let _remainder = 43 % 5;

    // Negative integers always truncates towards 0
    let _negative_rounding = (-7) / 4;
    println!("-7/4 rounds to {}", _negative_rounding);

    // % behaves like rem instead of mod (ie. always truncates towards 0)
    println!("-20 % 3 == {}", -20 % 3);
    println!("-5 % 3 == {}", -5 % 3);

    println!("---");

    // Booleans are 1 byte in size. While it's possible to pack many of them into a single
    // structure (eg. a number), the 7 bits lost per boolean aren't usually worth the complexity of
    // packing & unpacking.
    let _t = true;
    let _f: bool = false;

    // println!("---");

    let _c = 'z';
    let _z = 'ℤ';
    let _heart_eyed_cat = '😻';
    let ch = '\u{0041}';
    println!("\\u{{0041}} is '{}'", ch);

    println!("---");

    // tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (_x, y, _z) = tup;
    println!("The value of y is: {}", y);

    let x: (i32, f64, u8) = (500, 6.4, 1);
    let _five_hundred = x.0;
    let _six_point_four = x.1;
    let _one = x.2;

    println!("---");

    // arrays
    let _a = [1, 2, 3, 4, 5];
    let _months = ["January", "February", "March", "April", "May", "June", "July",
                "August", "September", "October", "November", "December"];
    let _a: [i32; 5] = [1, 2, 3, 4, 5];
    let _a = [3; 5];
    let _second = _a[1];
}
