use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fs;

use crate::english::ALPHABET;
use crate::fitness::{
    FitnessMatrix,
    compute_fitness,
    generate_fitness_matrix_from_file};

fn generate_key() -> [char; 26] {
    let mut key = ALPHABET.clone();
    key.shuffle(&mut thread_rng());
    key
}

fn encrypt(plaintext: &str, key: &[char; 26]) -> Result<String, String> {
    let mut ciphertext = String::new();
    for plaintext_char in plaintext.chars() {
        if plaintext_char == ' ' {
            ciphertext.push(' ');
            continue;
        }
        if plaintext_char < 'a' || plaintext_char > 'z' {
            return Err(format!("Char '{plaintext_char}' is invalid"));
        }
        let idx = plaintext_char as usize - 97;
        ciphertext.push(key[idx]);
    }
    Ok(ciphertext)
}

fn decrypt(ciphertext: &str, rev_key: &[char; 26]) -> Result<String, String> {
    encrypt(ciphertext, rev_key)
}

fn generate_char_positions(text: &str) -> Vec<Vec<usize>> {
    let mut char_positions: Vec<Vec<usize>> = Vec::new();
    for alphabet_char in ALPHABET {
        let mut positions: Vec<usize> = Vec::new();
        for (idx, text_char) in text.chars().enumerate() {
            if text_char == alphabet_char {
                positions.push(idx);
            }
        }
        char_positions.push(positions);
    }
    char_positions
}

fn hill_climb(ciphertext: &str, rev_key: &[char; 26], 
                matrix: &FitnessMatrix) -> (f64, [char; 26]) 
{
    let char_positions = generate_char_positions(ciphertext);
    let mut plaintext: Vec<usize> = decrypt(ciphertext, rev_key)
        .unwrap()
        .chars()
        .map(|s| if s == ' ' { 26 } else { s as usize - 97})
        .collect();
    let mut key = rev_key.clone();

    let mut current = compute_fitness(&plaintext, matrix);
    loop {
        let mut better_key = false;
        for to in 0..key.len() {
            for from in to+1..key.len() {
                let c1 = key[to] as usize - 97;
                let c2 = key[from] as usize - 97;

                for pos in &char_positions[to] {
                    plaintext[*pos] = c2;
                }
                for pos in &char_positions[from] {
                    plaintext[*pos] = c1;
                }

                let proposal = compute_fitness(&plaintext, matrix);
                if proposal > current {
                    key.swap(to, from);
                    current = proposal;
                    better_key = true;
                } else {
                    for pos in &char_positions[to] {
                        plaintext[*pos] = c1;
                    }
                    for pos in &char_positions[from] {
                        plaintext[*pos] = c2;
                    }
                }
            }
        }
        if !better_key { break };
    }
    (current, key)
}

fn crack(ciphertext: &str) -> Result<(f64, [char; 26]), &str> {
    if ciphertext.len() < 10 || ciphertext.len() > 1000000 {
        return Err("Length of cipher must in range 10..=1000000");
    }
    let matrix = generate_fitness_matrix_from_file();
    let mut local_maximum = 0.0;
    let mut local_maximum_hit = 0;
    let mut best_key: [char; 26] = ['a'; 26];
    for i in 1..10000 {
        if i % 100 == 0 {
            println!("{i} processed");
        }
        let new_rev_key = generate_key();
        let (fitness, rev_key) = hill_climb(ciphertext, &new_rev_key, &matrix);
        if fitness > local_maximum {
            local_maximum = fitness;
            best_key = rev_key;
        } else if fitness == local_maximum {
            local_maximum_hit += 1;
            if local_maximum_hit == 3 {
                break;
            }
        }
    }
    Ok((local_maximum, best_key))
}

pub fn main(args: &[String]) {
    let mut arg_encrypt = false;
    let mut arg_crack = false;
    match args.len() {
        1 => arg_crack = true,
        2 => {
            match args[1].as_str() {
                "encrypt" => arg_encrypt = true,
                "crack" => arg_crack = true,
                _ => panic!("Invalid argument {}", args[1]),
            }
        },
        _ => panic!("Invalid arguments"),
    }
    if arg_encrypt {
        let file_content = fs::read_to_string("plaintext.txt").unwrap();
        let plaintext = file_content.trim();
        let key = generate_key();
        let ciphertext = encrypt(&plaintext, &key).unwrap();
        fs::write("ciphertext.txt", ciphertext.as_bytes()).unwrap();

        println!("Plaintext: {}", plaintext);
        println!("Ciphertext: {}", ciphertext);
        println!("Key: {}", 
            key.iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(""));
    } else if arg_crack {
        let file_content = fs::read_to_string("ciphertext.txt").unwrap();
        let ciphertext = file_content.trim();
        let (fitness, best_key) = crack(&ciphertext).unwrap();
        let plaintext = decrypt(&ciphertext, &best_key).unwrap();

        println!("Cipher: {}", ciphertext);
        println!("Best reversed key: {}", 
            best_key.iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(""));
        println!("Best plaintext: {}", plaintext);
        println!("Fitness: {}", fitness);
    }
}
