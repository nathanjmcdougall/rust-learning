/*
Convert temperatures between degrees Fahrenheit and Celsius.

Reference:
https://en.wikipedia.org/wiki/Conversion_of_scales_of_temperature
*/

use std::io;

// Enums definitions are just comma separated element names
enum ConversionMode {
    CtoF,
    FtoC,
}

fn main() {
    let mode = get_conversion_mode_from_user();
    println!("Converting {}", mode.describe());

    let val = get_value_to_convert_from_user(&mode);
    let converted = match mode {
        // N.B. enum member access is like namespace access i.e. ::
        ConversionMode::CtoF => celsius_to_fahrenheit(val),
        ConversionMode::FtoC => fahrenheit_to_celsius(val),
    };
    println!(
        "{}{} = {}{}",
        val,
        mode.input_unit(),
        converted,
        mode.output_unit()
    );
}

fn fahrenheit_to_celsius(degrees_f: f64) -> f64 {
    (degrees_f - 32.0) / (9.0 / 5.0)
}

fn celsius_to_fahrenheit(degrees_c: f64) -> f64 {
    (degrees_c * (9.0 / 5.0)) + 32.0
}

fn get_conversion_mode_from_user() -> ConversionMode {
    let mode;
    loop {
        println!("Specify target unit (C/F):");
        println!("(C) : {}", ConversionMode::FtoC.describe());
        println!("(F) : {}", ConversionMode::CtoF.describe());
        let mut mode_choice = String::new();
        match io::stdin().read_line(&mut mode_choice) {
            Ok(mode_choice) => (),
            Err(_) => {
                _request_new_mode_choice();
                continue;
            }
        };
        let mode_str: char = match mode_choice.to_string().trim().parse() {
            Ok(mode_str) => mode_str,
            Err(_) => {
                _request_new_mode_choice();
                continue;
            }
        };
        mode = if mode_str == 'C' {
            ConversionMode::FtoC
        } else if mode_str == 'F' {
            ConversionMode::CtoF
        } else {
            _request_new_mode_choice();
            continue;
        };

        return mode;
    }
}
fn _request_new_mode_choice() {
    println!("Invalid choice, please try again. Choose either C or F.");
}

fn get_value_to_convert_from_user(mode: &ConversionMode) -> f64 {
    let val: f64;
    loop {
        let input_unit = mode.input_unit();
        println!("Please enter a value in {input_unit}:");
        let mut val_choice = String::new();
        match io::stdin().read_line(&mut val_choice) {
            Ok(val_choice) => (),
            Err(_) => {
                _request_new_val_choice(&mode);
                continue;
            }
        };
        val = match val_choice.to_string().trim().parse() {
            Ok(val) => val,
            Err(_) => {
                _request_new_val_choice(&mode);
                continue;
            }
        };

        return val;
    }
}
fn _request_new_val_choice(mode: &ConversionMode) {
    println!(
        "Invalid choice, please try again. Choose a number in {}.",
        mode.input_unit()
    );
}

/*
You can add methods to enums in a different place from the declaration of the enum
itself. Perhaps similar in spirit to the user of typing stubs in Python.
*/
impl ConversionMode {
    fn input_unit(&self) -> &str {
        match self {
            Self::CtoF => "°C",
            Self::FtoC => "°F",
        }
    }
    fn output_unit(&self) -> &str {
        match self {
            Self::CtoF => "°F",
            Self::FtoC => "°C",
        }
    }
    fn describe(&self) -> &str {
        match self {
            Self::CtoF => "°C -> °F",
            Self::FtoC => "°F -> °C",
        }
    }
}
