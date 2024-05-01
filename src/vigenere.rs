use std::fs;
use std::collections::HashSet;
use rand::thread_rng;
use rand::seq::SliceRandom;

use crate::english::ALPHABET;
use crate::fitness::{
    FitnessMatrix,
    generate_fitness_matrix_from_file,
    compute_fitness};

fn generate_key(length: usize) -> String {
    let mut key = String::new();
    let mut rng = thread_rng();
    for _ in 0..length {
        let random_char = ALPHABET.choose(&mut rng).unwrap();
        key.push(*random_char);
    }
    key
}

fn common<F>(source: &str, key: &str, calc_idx: F) -> String
    where F: Fn(usize, usize) -> usize 
{
    let mut transformed = String::new();
    let text: Vec<usize> = source.chars().map(|x| x as usize - 97).collect();
    let key: Vec<usize> = key.chars().map(|x| x as usize - 97).collect();
    for i in 0..text.len() {
        let j = i % key.len();
        let idx = calc_idx(text[i], key[j]);
        transformed.push(ALPHABET[idx]);
    }
    transformed
}

fn encrypt(plaintext: &str, key: &str) -> String {
    common(plaintext, key, |x, y| (x + y) % 26)
}

fn decrypt(ciphertext: &str, key: &str) -> String {
    common(ciphertext, key, |x, y| (x + 26 - y) % 26)
}

fn find_key_lengths(ciphertext: &str) -> Result<Vec<usize>, &str> {
    let mut bigram_distances: Vec<HashSet<usize>> = Vec::new();
    let mut bigrams: Vec<&str> = Vec::new();
    for bigram_idx in 0..ciphertext.len()-1 {
        let bigram = &ciphertext[bigram_idx..bigram_idx+2];
        if bigrams.contains(&bigram) {
            continue;
        }
        bigrams.push(bigram);

        let positions: Vec<usize> = ciphertext
            .match_indices(bigram)
            .map(|(bigram_idx, _)| bigram_idx)
            .collect();
        if positions.len() < 2 {
            continue;
        }

        let mut distances: HashSet<usize> = HashSet::new();
        for pos in 0..positions.len()-1 {
            let distance = positions[pos+1]-positions[pos];
            for factor in 1..=distance {
                if distance % factor == 0 {
                    distances.insert(factor);
                }
            }
        }
        bigram_distances.push(distances);
    }
    if bigram_distances.len() < 3 {
        return Err("Not enough bigram");
    }
    bigram_distances.sort_by(|a, b| b.len().cmp(&a.len()));

    let mut key_lengths: HashSet<usize> = bigram_distances[0].clone();
    for distances in &bigram_distances[1..3] {
        key_lengths = key_lengths
            .intersection(&distances)
            .map(|x| *x)
            .collect();
    }

    let mut key_lengths: Vec<usize> = key_lengths.into_iter().collect();
    key_lengths.sort();
    Ok(key_lengths)
}

fn hill_climb(ciphertext: &str, key: &str, matrix: &FitnessMatrix) -> (f64, String) {
    let mut plaintext: Vec<u8> = decrypt(ciphertext, key)
        .chars()
        .map(|x| x as u8 - 97)
        .collect();
    let mut key: Vec<char> = key.chars().collect();

    let ciphertext: Vec<u8> = ciphertext.chars()
        .map(|x| x as u8 - 97)
        .collect();
        
    let mut current = compute_fitness(&plaintext, matrix);
    loop {
        let mut better_key = false;
        for key_idx in 0..key.len() {
            for alphabet_char in ALPHABET {
                let alphabet_bin = alphabet_char as u8 - 97;
                for idx in (key_idx..plaintext.len()).step_by(key.len()) {
                    plaintext[idx] = (ciphertext[idx]+26-alphabet_bin) % 26;
                }

                let proposal = compute_fitness(&plaintext, matrix);
                if proposal > current {
                    key[key_idx] = alphabet_char;
                    current = proposal;
                    better_key = true;
                } else {
                    let key_bin = key[key_idx] as u8 - 97;
                    for idx in (key_idx..plaintext.len()).step_by(key.len()) {
                        plaintext[idx] = (ciphertext[idx]+26-key_bin) % 26;
                    }
                }
            }
        }
        if !better_key { break; }
    }
    (current, key.iter().collect::<String>())
}

fn crack(ciphertext: &str) -> (f64, String) {
    if ciphertext.len() < 150 {
        panic!("Ciphertext minium len is 150");
    }
    let matrix = generate_fitness_matrix_from_file();
    let key_lengths: Vec<usize> = if let Ok(x) = find_key_lengths(ciphertext) {
        x
    } else {
        (1..ciphertext.len()).collect()
    };
    
    let mut local_maximum = -1000.0;
    let mut local_maximum_hit = 0;
    let mut best_key: String = String::new();
    for length in key_lengths {
        println!("Length: {length}");
        let new_key = generate_key(length);
        let (fitness, key) = hill_climb(&ciphertext, &new_key, &matrix);

        if fitness > local_maximum {
            local_maximum = fitness;
            best_key = key;
        } else if fitness == local_maximum {
            local_maximum_hit += 1;
            if local_maximum_hit == 3 {
                break;
            }
        }
    }
    (local_maximum, best_key)
}

pub fn main() {
    let file_content = fs::read_to_string("plaintext.txt").unwrap();
    let plaintext = file_content.trim().replace(" ", "");
    let key = "helloworld";
    let ciphertext = encrypt(&plaintext, &key);

    let (fitness, best_key) = crack(&ciphertext);
    let decrypted = decrypt(&ciphertext, &best_key);
    println!("Key: {}", key);
    println!("Guess key: {}", best_key);
    println!("Fitness: {}", fitness);
    println!("Decrypted: {}", decrypted);
}
