use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::english::ALPHABET;
use crate::fitness::{
    FitnessMatrix,
    compute_fitness,
    generate_fitness_matrix_from_file};

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
        let rotor1 = &rotated_key[0][idx%26];
        let rotor2 = &rotated_key[1][idx/26%26];
        let rotor3 = &rotated_key[2][idx/(26*26)%26];

        let plaintext_bin = plaintext_char as usize - 97;
        let round1 = rotor1[plaintext_bin] as usize - 97;
        let round2 = rotor2[round1] as usize - 97;
        let round3 = rotor3[round2];
        ciphertext.push(round3);
    }
    Ok(ciphertext)
}

fn reverse_key(key: &[[[char; 26]; 26]; 3]) -> [[[char; 26]; 26]; 3] {
    fn reverse_rotor(rotor: &[char; 26]) -> [char; 26] {
        let mut rev_rotor = ['a'; 26];
        for (idx, value) in rotor.iter().enumerate() {
            let to = ALPHABET[idx];
            let from = *value as usize - 97;
            rev_rotor[from] = to;
        }
        rev_rotor
    }
    let mut rev = key.clone();
    for i in 0..3 {
        for j in 0..26 {
            rev[i][j] = reverse_rotor(&rev[i][j]);
        }
    }
    rev
}

fn decrypt(ciphertext: &str, key: &EnigmaKey) -> Result<String, String> {
    let rotated_key = rotate_key(&key);
    let rotated_key = reverse_key(&rotated_key);
    let mut plaintext = String::new();
    for (idx, ciphertext_char) in ciphertext.chars().enumerate() {
        let rotor1 = &rotated_key[0][idx%26];
        let rotor2 = &rotated_key[1][idx/26%26];
        let rotor3 = &rotated_key[2][idx/(26*26)%26];

        let ciphertext_bin = ciphertext_char as usize - 97;
        let round1 = rotor3[ciphertext_bin] as usize - 97;
        let round2 = rotor2[round1] as usize - 97;
        let round3 = rotor1[round2];
        plaintext.push(round3);
    }
    Ok(plaintext)
}

fn fast_decrypt(ciphertext: &str, 
    rotated_key: &[[[char; 26]; 26]; 3],
    plaintext: &mut Vec<u8>) 
{
    for (idx, ciphertext_char) in ciphertext.chars().enumerate() {
        let rotor1 = &rotated_key[0][idx%26];
        let rotor2 = &rotated_key[1][idx/26%26];
        let rotor3 = &rotated_key[2][idx/(26*26)%26];

        let ciphertext_bin = ciphertext_char as usize - 97;
        let round1 = rotor3[ciphertext_bin] as usize - 97;
        let round2 = rotor2[round1] as usize - 97;
        let round3 = rotor1[round2] as u8 - 97;

        plaintext[idx] = round3;
    }
}

fn fast_swap_key(rotated_key: &mut [[[char; 26]; 26]; 3], 
    rotor_num: usize, char_from: char, char_to: char) 
{
    let from = char_from as usize - 97;
    let to = char_to as usize - 97;
    rotated_key[rotor_num].iter_mut()
        .for_each(move |key| key.swap(from, to));
}


fn hill_climb(ciphertext: &str, init_key: &EnigmaKey, 
                matrix: &FitnessMatrix) -> (f64, EnigmaKey) 
{
    let mut plaintext: Vec<u8> = decrypt(ciphertext, init_key)
        .unwrap()
        .chars()
        .map(|x| x as u8 - 97)
        .collect();
    let mut vec_key = [init_key.0, init_key.1, init_key.2];
    let mut rotated_key = reverse_key(&rotate_key(&init_key));

    let mut current = compute_fitness(&plaintext, matrix);
    loop {
        let mut better_key = false;
        for rotor_num in 0..3 {
            for from in 0..26 {
                for to in from+1..26 {
                    fast_swap_key(&mut rotated_key, rotor_num, 
                        vec_key[rotor_num][from], 
                        vec_key[rotor_num][to]);
                    fast_decrypt(ciphertext, &rotated_key, &mut plaintext);

                    let proposal = compute_fitness(&plaintext, matrix);
                    if proposal > current {
                        vec_key[rotor_num].swap(to, from);
                        current = proposal;
                        better_key = true;
                    } else {
                        fast_swap_key(&mut rotated_key, rotor_num, 
                            vec_key[rotor_num][from], 
                            vec_key[rotor_num][to]);
                    }
                }
            }
        }
        if !better_key { break; }
    }
    let key = EnigmaKey(vec_key[0], vec_key[1], vec_key[2]);
    (current, key)
}

fn crack(ciphertext: &str) -> Result<(f64, EnigmaKey), &str> {
    if ciphertext.len() < 10 || ciphertext.len() > 1000000 {
        return Err("Length of cipher must in range 10..=1000000");
    }
    let matrix = generate_fitness_matrix_from_file();
    let mut local_maximum = 0.0;
    let mut local_maximum_hit = 0;
    let mut best_key = generate_enigmakey();
    for i in 1..1000000 {
        if i % 1000 == 0 {
            println!("{i} processed {:.4}", local_maximum);
        }

        let init_key = generate_enigmakey();
        let (fitness, rev_key) = hill_climb(ciphertext, &init_key, &matrix);

        if fitness > local_maximum {
            local_maximum = fitness;
            best_key = rev_key;
        } else if fitness == local_maximum {
            local_maximum_hit += 1;
        }
        if local_maximum_hit == 100 {
            break;
        }
    }
    Ok((local_maximum, best_key))
}

pub fn main() {
    let plaintext = "to be or not to be that is the question"
        .replace(" ", "");
    let key = generate_enigmakey();

    /*
    let mut rotated_key = reverse_key(&rotate_key(&init_key));

    init_key.0.swap(1, 5);
    let swap_rotated_key = reverse_key(&rotate_key(&init_key));
    
    fast_swap_key(&mut rotated_key, 0, init_key.0[1], init_key.0[5]);

    assert_eq!(swap_rotated_key, rotated_key);
    */

    let ciphertext = encrypt(&plaintext, &key).unwrap();

    let (fitness, guess_key) = crack(&ciphertext).unwrap();
    println!("Fitness: {}", fitness);
    println!("{}", decrypt(&ciphertext, &guess_key).unwrap());
}
