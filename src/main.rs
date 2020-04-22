use std::io;

fn celsius_to_fahrenheit(value: u32) -> f32 {
    value as f32 * 9.0 / 5.0 + 32.0
}

fn fahrenheit_to_celsius(value: u32) -> f32 {
    (value as f32 - 32.0) * 5.0 / 9.0
}

fn main() {
    let c_to_f = "Celsius to Fahrenheit";
    let f_to_c = "Fahrenheit to Celsius";

    println!("Choose convertion:");
    println!("1) {}", c_to_f);
    println!("2) {}", f_to_c);

    let convertion: u32 = loop {
        let mut option = String::new();
        io::stdin()
            .read_line(&mut option)
            .expect("Failed to read line");
        match option.trim().parse() {
            Ok(num) => {
                if num != 1 && num != 2 {
                    println!("Not a valid option");
                    continue;
                } else {
                    break num;
                }
            }
            Err(_) => {
                println!("Not a number");
                continue;
            }
        };
    };
    let celsius: bool = { convertion == 1 };
    println!("You choose {}", if celsius { c_to_f } else { f_to_c });

    loop {
        println!("Choose the value");
        let mut value = String::new();
        io::stdin()
            .read_line(&mut value)
            .expect("Failed to read line");
        let value: u32 = match value.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("{} is not a number", value.trim());
                continue;
            }
        };
        break println!(
            "{} {} is {}",
            value,
            if celsius { c_to_f } else { f_to_c },
            if celsius {
                celsius_to_fahrenheit(value)
            } else {
                fahrenheit_to_celsius(value)
            },
        );
    }
}
