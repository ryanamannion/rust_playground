use std::io;

const FACTOR: f32 = 5.0/9.0;

fn f_to_c(temp: f32) -> f32 {
    (temp - 32.0) * FACTOR
}

fn c_to_f(temp: f32) -> f32 {
    (temp / FACTOR) + 32.0
}

fn main() {
    // convert temperatures from Fahrenheit to Celsius

    println!("Please enter a temperature to convert.");

    let mut original = String::new();

    io::stdin()
        .read_line(&mut original)
        .expect("Faled to read line");

    let original = original.trim().parse::<f32>().expect("Failed to parse number");

    println!("Is this temperature fahrenheit [f] or celsius [c]?");

    let mut original_unit = String::new();

    io::stdin()
        .read_line(&mut original_unit)
        .expect("Failed to read line");

    let original_unit = original_unit.to_lowercase();
    let original_unit = original_unit.trim();

    println!("original: {original} unit: {original_unit}");

    let converted = if original_unit == "f" {
        f_to_c(original)
    } else {
        c_to_f(original)
    };

    println!("{original:.2}°{original_unit} converted is {converted:.2}°");

}
