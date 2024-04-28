use crate::english::{ALPHABET, ENGLISH_FREQ};

fn encrypt(plaintext: &str, key: usize) -> Result<String, String> {
    if key < 1 || key > 26 {
        return Err(format!("Key {key} is out of range 1..=26"));
    }
    let mut ciphertext = String::new();
    for plaintext_char in plaintext.chars() {
        let from = ALPHABET.iter()
            .position(|&alphabet_char| alphabet_char == plaintext_char)
            .expect(&format!("Char '{plaintext_char}' is out of range a..=z"));
        let to = (from+26-key) % 26;
        ciphertext.push(ALPHABET[to]);
    }
    Ok(ciphertext)
}

fn decrypt(ciphertext: &str, key: usize) -> Result<String, String> {
    if key < 1 || key > 26 {
        return Err(format!("Key {key} is out of range 1..=26"));
    }
    let mut plaintext = String::new();
    for ciphertext_char in ciphertext.chars() {
        let from = ALPHABET.iter()
            .position(|&alphabet_char| alphabet_char == ciphertext_char)
            .expect(&format!("Char '{ciphertext_char}' is out of range a..=z"));
        let to = (from+key) % 26;
        plaintext.push(ALPHABET[to]);
    }
    Ok(plaintext)
}

fn counter(text: &str) -> Vec<usize> {
    let mut counts: Vec<usize> = Vec::new();
    for alphabet_char in ALPHABET {
        let number_occur = text.matches(alphabet_char).count(); 
        counts.push(number_occur);
    }
    counts
}

fn chi_sqr(text: &str, expect_freq: &[f64; 26]) -> f64 {
    let counts = counter(text);
    let text_len = text.len() as f64;
    let chi: f64 = counts.into_iter().zip(expect_freq)
        .map(|(c, f)| if c == 0 { 0.0 } 
        else {
            ((c as f64)-text_len*f).powi(2) / (text_len*f)
        }).sum();
    chi
}


fn crack(cipher: &str, expect_freq: &[f64; 26]) -> Vec<(f64, usize)> {
    let mut cracks: Vec<(f64, usize)> = (1..27).into_iter()
        .map(|i| {
            let decrypted = decrypt(cipher, i).unwrap();
            let chi = chi_sqr(&decrypted, expect_freq);
            (chi, i)
        }).collect();
    cracks.sort_by(|(c1, _), (c2, _)| c1.partial_cmp(c2).unwrap());
    cracks
}

fn print_cracks(cracks: &Vec<(f64, usize)>, cipher: &str) {
    for (chi, key) in &cracks[..5] {
        println!("{} {:>2} {}", 
            decrypt(cipher, *key).unwrap(),
            key, 
            chi);
    }
}

pub fn main() {
    let text = "helloworld";
    let ciphertext = encrypt(text, 23).unwrap();
    let plaintext = decrypt(&ciphertext, 23).unwrap();
    println!("Ciphertext: {}", ciphertext);
    println!("Plaintext: {}", plaintext);

    let cracks = crack(&ciphertext, &ENGLISH_FREQ);
    print_cracks(&cracks, &ciphertext);
}
