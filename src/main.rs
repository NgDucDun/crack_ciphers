use std::env;

mod caesar;
mod enigma;
mod english;
mod fitness;
mod vigenere;
mod substitution;

fn main() {
    let args: Vec<String> = env::args().collect();
    let error_msg = "Invalid argument (use help for help)";
    let help_msg = "Usage: ciphers <arguments>
arguments: 
    help 
    caesar 
    enigma
    vigenere 
    substitution
    make_fitness_file";
    match args.len() {
        1 => println!("{}", error_msg),
        _ => match args[1].as_str() {
            "help" => println!("{}", help_msg),
            "caesar" => caesar::main(),
            "enigma" => enigma::main(),
            "vigenere" => vigenere::main(),
            "substitution" => substitution::main(&args[1..]),
            "make_fitness_file" => fitness::make_fitness_matrix_file(),
            _ => println!("{}", error_msg),
        }
    }
}
