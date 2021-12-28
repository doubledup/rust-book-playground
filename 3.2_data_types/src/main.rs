fn main() {
    println!("Hello, world!");

    // println!("{}", 3_45i32 + 1_23u8);

    let _x = 2.0;
    let _y = 3.0;

    let _sum = 5 + 10;
    let _difference = 95.5 - 4.3;
    let _product = 4 * 30;
    let _quotient = 56.7 / 32.2;
    let _floored = 2 / 3;
    let _remainder = 43 % 5;

    println!("{}", -20 % 3);

    let _t = true;
    let _f: bool = false;

    let _c = 'z';
    let _z = 'ℤ';
    let _heart_eyed_cat = '😻';

    // tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (_x, y, _z) = tup;
    println!("The value of y is: {}", y);

    let x: (i32, f64, u8) = (500, 6.4, 1);
    let _five_hundred = x.0;
    let _six_point_four = x.1;
    let _one = x.2;

    // arrays
    let _a = [1, 2, 3, 4, 5];
    let _months = ["January", "February", "March", "April", "May", "June", "July",
                "August", "September", "October", "November", "December"];
    let _a: [i32; 5] = [1, 2, 3, 4, 5];
    let _a = [3; 5];
    let _second = _a[1];
}
