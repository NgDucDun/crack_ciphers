use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fs;

use crate::english::ALPHABET;
/*
use crate::fitness::{
    FitnessMatrix,
    compute_fitness,
    generate_fitness_matrix_from_file};
*/

struct EnigmaKey([char; 26], [char; 26], [char; 26]);

fn generate_key() -> [char; 26] {
    let mut key = ALPHABET.clone();
    key.shuffle(&mut thread_rng());
    key
}

fn generate_enigmakey() -> EnigmaKey {
    EnigmaKey(generate_key(),
        generate_key(),
        generate_key())
}

fn rotate_key(key: &EnigmaKey) -> [[[char; 26]; 26]; 3] {
    let mut rotated_key = [[['a'; 26]; 26]; 3];
    let vec_key = [key.0, key.1, key.2];
    for rotor_num in 0..3 {
        for rotate_num in 0..26 {
            let mut rotor = vec_key[rotor_num].clone();
            rotor.rotate_left(rotate_num);
            rotated_key[rotor_num][rotate_num] = rotor;
        }
    }
    rotated_key
}

fn encrypt(plaintext: &str, key: &EnigmaKey) -> Result<String, String> {
    let rotated_key = rotate_key(key);
    let mut ciphertext = String::new();
    for (idx, plaintext_char) in plaintext.chars().enumerate() {
        if plaintext_char < 'a' || plaintext_char > 'z' {
            return Err(format!("Invalid character {plaintext_char} in plaintext"));
        }
        let plaintext_bin = plaintext_char as usize - 97;
        let rotor1 = &rotated_key[0][idx%26];
        let rotor2 = &rotated_key[1][idx/26%26];
        let rotor3 = &rotated_key[2][idx/(26*26)%26];

        let round1 = rotor1[plaintext_bin] as usize - 97;
        let round2 = rotor2[round1] as usize - 97;
        let round3 = rotor3[round2];
        ciphertext.push(round3);
    }
    Ok(ciphertext)
}

/*
fn reverse_key(key: &EnigmaKey) -> EnigmaKey {
    fn reverse_rotor(rotor: &[char; 26]) -> [char; 26] {
        let mut rev_rotor = ['a'; 26];
        for (idx, value) in rotor.iter().enumerate() {
            let to = ALPHABET[idx];
            let from = *value as usize - 97;
            rev_rotor[from] = to;
        }
        rev_rotor
    }
    EnigmaKey(reverse_rotor(&key.0),
        reverse_rotor(&key.1),
        reverse_rotor(&key.2))
}

fn decrypt(ciphertext: &str, key: &EnigmaKey) -> Result<String, String> {
    let rev_key = reverse_key(key);
    let third = sub(ciphertext, &rev_key.2)?;
    let second = sub(&third, &rev_key.1)?;
    let first = sub(&second, &rev_key.0)?;
    Ok(first)
}
*/

pub fn main() {
    let plaintext = fs::read_to_string("plaintext.txt")
        .unwrap()
        .trim()
        .replace(" ", "");
    let key = generate_enigmakey();

    let ciphertext = encrypt(&plaintext, &key).unwrap();
    println!("Ciphertext: {}", ciphertext);
}
