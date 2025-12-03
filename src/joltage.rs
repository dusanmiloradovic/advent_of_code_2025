use std::fs::read_to_string;
const DIGITS: usize = 12;

fn find_two_digit_joltage(v: &str) -> u32 {
    let mut first_dig: u32 = 0;
    let mut second_dig: u32 = 0;

    for i in 0..v.len() - 1 {
        let dig1 = v[i..i + 1].parse::<u32>().unwrap();
        let dig2 = v[i + 1..i + 2].parse::<u32>().unwrap();
        if dig1 > first_dig {
            first_dig = dig1;
            second_dig = dig2;
            continue;
        }
        if dig2 > second_dig {
            second_dig = dig2;
        }
    }
    first_dig * 10 + second_dig
}

fn copy_slice_from(
    line: &[u128],
    joltage_vec: &mut [u128],
    line_index: usize,
    joltage_index: usize,
) {
    for j in joltage_index..DIGITS {
        joltage_vec[j] = line[j + line_index];
    }
}

fn find_twelve_digit_joltage(v: &str) -> u128 {
    let mut joltage_line: Vec<u128> = vec![0; v.len()];
    for i in 0..v.len() {
        joltage_line[i] = v[i..i + 1].parse::<u128>().unwrap();
    }
    let mut joltage_vec: Vec<u128> = vec![0; DIGITS];
    for i in 0..v.len() {
        let mut offset = DIGITS as i32 - (v.len() - i) as i32;
        if offset < 0 {
            offset = 0;
        }
        for j in 0..DIGITS - offset as usize {
            if joltage_line[i + j] > joltage_vec[j + offset as usize] {
                joltage_vec[j + offset as usize] = joltage_line[i + j];
                copy_slice_from(&joltage_line, &mut joltage_vec, i, j + 1 + offset as usize);
            }
        }
    }
    let mut joltage: u128 = 0;
    let base: u128 = 10;
    for i in 0..DIGITS {
        joltage += base.pow((DIGITS - i - 1) as u32) * joltage_vec[i];
    }
    joltage
}
pub fn find_joltage() {
    let mut rez: u32 = 0;
    let lines = get_puzzle_lines();
    for l in lines {
        let j = find_two_digit_joltage(&l);
        rez += j;
        // print!("{j}\n");
    }
    print!("joltage={rez}\n");
}

pub fn find_12_digits_joltage() {
    let mut rez: u128 = 0;
    let lines = get_puzzle_lines();
    for l in lines {
        let j = find_twelve_digit_joltage(&l);
        rez += j;
        //print!("{j}\n");
    }
    print!("12 digits joltage={rez}");
}

fn get_puzzle_lines() -> Vec<String> {
    let lines: Vec<String> = read_to_string("puzzle_input_day3.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();
    lines
}
