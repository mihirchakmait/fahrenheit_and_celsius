use std::io;

fn main() {
    loop {
        println!("Please enter 0 for fahrenheit to celcius, or 1 for celcius to fahrenheit.");
        let input_f32: f32 = read_input_line_f32();
        println!("");
        if input_f32 == 0.0 {
            println!("Converting from fahrenheit to celcius");
            println!("Please enter number of degrees fahrenheit that you want to convert to degrees to celcius.");
            let fahrenheit_input: f32 = read_input_line_f32();
            let celcius: f32 = fahrenheit_to_celcius(fahrenheit_input);
            println!("{} degrees fahrenheit is {} degrees celcius.", fahrenheit_input, celcius);
        } else {
            println!("Coverting from celcius to fahrenheit.");
            println!("Please enter celcius degrees that you want to convert.");
            let celcius_input: f32 = read_input_line_f32();
            let fahrenheit = celcius_to_fahrenheit(celcius_input);
            println!("{} degrees celcius is {} degrees fahrenheit.", celcius_input, fahrenheit);
        }
        println!("");
    }
}

fn read_input_line_f32() -> f32 {
    loop {
        let mut input_string = String::new();
        println!("Enter number: ");
        
        io::stdin().read_line(&mut input_string)
            .expect("Failed to read line.");
            
        let _input_string: f32 = match input_string.trim().parse() {
            Ok(num) => return num,
            Err(_) => continue,
        };
    }
}

fn celcius_to_fahrenheit(celcius: f32) ->f32 {
    (celcius * 1.8) + 32.0
}

fn fahrenheit_to_celcius(fahrenheit: f32) -> f32 {
    (fahrenheit - 32.0) / 1.8
}
