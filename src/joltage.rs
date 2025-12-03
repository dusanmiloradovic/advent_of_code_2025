use std::fs::read_to_string;

fn find_two_digit_joltage(v: &str) -> u32 {
    let mut first_dig: u32 = 0;
    let mut second_dig: u32 = 0;

    for i in 0..v.len() - 1 {
        let dig1 = v[i..i+1].parse::<u32>().unwrap();
        let dig2 =  v[i+1..i+2].parse::<u32>().unwrap();
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
pub fn find_joltage() {
    let mut rez:u32=0;
    let lines: Vec<String> = read_to_string("puzzle_input_day3.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();
    for l in lines{
        let j = find_two_digit_joltage(&l);
        rez += j;
       // print!("{j}\n");
    }
    print!("Result={rez}");
}
