use inquire::Select;
use std::fmt;
use std::io;

#[derive(Debug, Copy, Clone)]
#[allow(clippy::upper_case_acronyms)]
enum Menu {
    Celsius,
    Fahrenheit,
}

impl Menu {
    // add code here
    const VARIANTS: &'static [Menu] = &[Self::Celsius, Self::Fahrenheit];
}

impl fmt::Display for Menu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Celsius => write!(f, "Convert temperature from Fahrenheit to Celsius"),
            Self::Fahrenheit => write!(f, "Convert temperature from Celsius to Fahrenheit"),
        }
    }
}

fn main() {
    let answer: Menu = Select::new("Select your action:", Menu::VARIANTS.to_vec())
        .with_page_size(9)
        .prompt()
        .unwrap_or_else(|_| std::process::exit(0));
    match answer {
        Menu::Celsius => loop {
            println!("Provide the temperature in Celsius");
            let mut degrees = String::new();
            io::stdin()
                .read_line(&mut degrees)
                .expect("Failed to read line");
            let degrees: f64 = match degrees.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };
            let degrees = degrees * 1.8 + 32.0;
            println!("Your temperature in Fahrenheit: {degrees}");
            break;
        },
        Menu::Fahrenheit => loop {
            println!("Provide the temperature in Fahrenheit");
            let mut degrees = String::new();
            io::stdin()
                .read_line(&mut degrees)
                .expect("Failed to read line");
            let degrees: f64 = match degrees.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };
            let degrees = (degrees - 32.0) * 0.5556;
            println!("Your temperature in Celsius: {degrees}");
            break;
        },
    }
}
