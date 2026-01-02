use std::io;

fn main() {
    loop {
        println!("1.F to C");
        println!("2.C to F");
        println!("What do you want to do?");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read the line");
        let choice: u8 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please try again");
                continue;
            }
        };

        println!("Please input temperature: ");
        let mut temperature = String::new();
        io::stdin()
            .read_line(&mut temperature)
            .expect("Failed to read the line");
        let temperature: f32 = match temperature.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Error while converting to integer");
                continue;
            }
        };

        if choice == 1 {
            let temperature: f32 = (temperature - 32.0) / 1.8;
            println!("Celsius: {temperature}");
        } else if choice == 2 {
            let temperature: f32 = (temperature * 1.8) + 32.0;
            println!("Fahrenheit: {temperature}");
        } else {
            println!("You entered an incorrect menu item");
            continue;
        }
    }
}
