use std::io;

fn main() {
    let mut temperature = String::new();

    let (number, unit): (u64, &str) = loop {
        println!("Enter a temperature as a number, followed by C for Celsius or F for Fahrenheit");
        println!("eg. 15C for 15 degrees Celsius or 105F for 105 degrees Fahrenheit");
        io::stdin()
            .read_line(&mut temperature)
            .expect("Failed to read line");
        // Remove trailing newline
        let temperature = temperature.trim();

        if temperature.len() < 2 {
            println!("Input is too short!");
            continue;
        }
        let (number, unit) = temperature.split_at(temperature.len() - 1);

        if !(unit == "C" || unit == "F") {
            println!("Unit not recognised. Please use C for Celsius or F for Fahrenheit");
            continue;
        }

        let number = match number.parse::<u64>() {
            Ok(number) => number,
            Err(_) => {
                println!("Invalid number '{}'.", number);
                continue;
            }
        };

        break (number, unit);
    };

    if unit == "F" {
        println!("{}C", (number - 32) * 5 / 9);
    } else if unit == "C" {
        println!("{}F", number * 9 / 5 + 32);
    };
}
