use std::io;
fn main() {
    loop {
        println!("Provide the temperature in Celsius");
        let b: f64 = 1.8;
        let c: f64 = 32.0;
        let mut dagrees = String::new();
        io::stdin()
            .read_line(&mut dagrees)
            .expect("Failed to read line");
        let dagrees: f64 = match dagrees.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        let dagrees = dagrees * b + c;
        println!("Your temperature in Fahrenheit: {dagrees}");
        break;
    }
}
