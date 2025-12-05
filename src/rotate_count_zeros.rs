use std::fs::read_to_string;
use crate::utils;
const DIALS: i32 = 100;
const INITIAL_DIAL: i32 = 50;
fn get_seq(inn: &[String]) -> Vec<i32> {
    let mut ret: Vec<i32> = vec![];
    for i in inn {
        let val = &i[1..];
        let mut v = val.parse::<i32>().unwrap();

        let c = i.chars().nth(0).unwrap();
        if c == 'L' {
            v *= -1
        }
        ret.push(v);
    }
    ret
}



fn count_zeros(rotations: &[i32]) -> u32 {
    let mut ret: u32 = 0;
    let mut dial = INITIAL_DIAL;
    for r in rotations {
        dial += r;
        if (dial % DIALS) == 0 {
            ret += 1;
        }
    }

    ret
}


fn count_zeros_including_passing_through(rotations: &[i32]) -> u32 {
    let mut ret: u32 = 0;
    let mut dial = INITIAL_DIAL;
    let mut old_dial = dial;
    for r in rotations {
      //  print!("dial {r} for {dial}");

        let full_s = (r / DIALS).abs() as u32;
        ret += full_s;
        let dd  = r % DIALS;
        dial += dd;
        dial = dial % DIALS;

        if dial == 0 && old_dial != 0 {
            ret += 1;
        } else {
            if (dial < 0) {
                dial = DIALS + dial;
            }

            if old_dial != 0 && *r > 0 && dial < old_dial {
                ret += 1;
            }
            if old_dial != 0 && *r < 0 && dial > old_dial {
                ret += 1;
            }
        }

       // print!(", after {dial},,old_dial={old_dial}, ret= {ret}\n");
        old_dial = dial;
    }
    ret
}

pub fn times_zero_dial(file_name: &str) {
    let file_lines = utils::read_file(file_name);
    let z = &file_lines;
    let seq_z = get_seq(z);
    let seq = seq_z.as_slice();
    let zeros = count_zeros_including_passing_through(seq);
    print!("How many zeros? {zeros} zeros\n");
}
