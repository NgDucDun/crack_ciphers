use std::fs;

fn generate_fitness_matrix(text: &str) -> Vec<f64> {
    let text_bin: Vec<u8> = text.chars()
        .map(|x| if x == ' ' { 26 } else { x as u8 - 97 })
        .collect();
    let mut quad_counts: Vec<u32> = vec![0; 32*32*32*32];
    for i in 0..text_bin.len()-3 {
        let idx = ((text_bin[i] as usize) << 15) +
            ((text_bin[i+1] as usize) << 10) +
            ((text_bin[i+2] as usize) << 5) +
            text_bin[i+3] as usize;
        quad_counts[idx] += 1;
    }
    let total = text_bin.len() - 3;
    
    let min_value = *quad_counts.iter()
        .filter(|x| **x != 0)
        .min()
        .unwrap();
    let offset = (min_value as f64 / 10.0 / total as f64).log10();

    let mut matrix: Vec<f64> = vec![0.0; 32*32*32*32];
    let mut norm = 0.0;
    for (idx, value) in quad_counts.into_iter().enumerate() {
        if value == 0 { continue; }
        let prop = value as f64 / total as f64;
        let new_value = prop.log10() - offset;
        norm += prop * new_value;

        matrix[idx] = new_value;
    }
    matrix.iter_mut().for_each(|x| *x = *x / norm * 100.0);
    matrix
}

pub fn make_fitness_matrix_file() {
    let file_content = fs::read_to_string("war_and_peace_processed.txt").unwrap();
    let matrix = generate_fitness_matrix(&file_content);
    println!("{}", matrix.len());
    let _ = fs::write(format!("fitness_matrix.data"),
        matrix.into_iter().map(|x| x.to_string())
        .collect::<Vec<String>>().join("\n"));
}

pub struct FitnessMatrix {
    // 32768 = 32^4 / 32
    matrix: [u64; 32768],
}

pub fn generate_fitness_matrix_from_file() -> FitnessMatrix {
    let matrix = fs::read_to_string("fitness_matrix.data")
        .unwrap()
        .split("\n")
        .map(|x| (x.parse::<f64>().unwrap()/100.0*2.0).round() as u8)
        .collect::<Vec<u8>>();
    let mut bit_matrix: [u64; 32768] = [0; 32768];
    let mut bit_matrix_idx = 0;
    for i in (0..matrix.len()).step_by(32) {
        let mut bit: u64 = 0;
        for j in 0..32 {
            bit |= (matrix[i+j] as u64) << (j*2);
        }
        bit_matrix[bit_matrix_idx] = bit;
        bit_matrix_idx += 1;
    }
    FitnessMatrix { matrix: bit_matrix }
}

pub fn compute_fitness(text: &Vec<u8>, matrix: &FitnessMatrix) -> f64 {
    let mut idx: usize = ((text[0] as usize) << 10) + 
        ((text[1] as usize) << 5) +
        text[2] as usize;
    let mut fitness = 0;
    for text_char in &text[3..] {
        idx = ((idx & 0x7FFF) << 5) | (*text_char as usize);
        //                         *32         mod 32
        let val = matrix.matrix[idx>>5] >> ((idx&0x1F) << 1) & 0x03;
        fitness += val as u64;
    }
    fitness as f64 / 2.0 * 100.0 / (text.len()-3) as f64
}
