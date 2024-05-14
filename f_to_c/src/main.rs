use std::io;

fn main() {
    println!("Enter a temperature in Fahrenheit: ");

    let mut input_temp = String::new();

    io::stdin()
        .read_line(&mut input_temp)
        .expect("Failed to parse input!");

    let input_temp: f32 = input_temp.trim().parse().expect("Please enter a valid temperature!!");

    println!("{}", (&input_temp - 32.0) / 1.8 );
}
