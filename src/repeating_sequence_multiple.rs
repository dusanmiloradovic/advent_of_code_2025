use std::cmp::max;
use std::collections::HashSet;
use std::fs::read_to_string;

fn invalid_in_range_with_digits(
    left_range: &str,
    right_range: &str,
    digits: usize,
) -> Option<Vec<u128>> {
    let base: u128 = 10;
    let left_num = left_range.parse::<u128>().unwrap();
    let right_num = right_range.parse::<u128>().unwrap();
    let mut normalized_left = left_range.to_string();
    let mut normalized_right = right_range.to_string();
    let satisfies_left = normalized_left.len() % digits == 0 && normalized_left.len()>1;
    let satisfies_right = normalized_right.len() % digits == 0 && normalized_right.len()>1;
    let mut ret: Vec<u128> = vec![];
    if !satisfies_left && !satisfies_right {
        // Here we rely on problem input, there will be no differences in keys with two orders of mangiture
        // (two digits more), the max diff is one digit, so if none of this is satisified we return None
        return None;
    }
    if !satisfies_left {
        normalized_left = base.pow(right_range.len() as u32 - 1).to_string();
    }
    if !satisfies_right {
        normalized_right = (base.pow(left_range.len() as u32) - 1).to_string();
    }

    let left_bound = normalized_left[..digits].parse::<u128>().unwrap();
    //let right_bound = normalized_right[..digits].parse::<u128>().unwrap();
    let right_bound = base.pow(digits as u32 +1);

    let mut no_of_digits = normalized_left.len();
    loop {
        let repeats = no_of_digits / digits;
        for x in left_bound..right_bound {
            let mut candidate = "".to_string();
            for _ in 0..repeats {
                candidate.push_str(x.to_string().as_str());
            }
            let can_num = candidate.parse::<u128>().unwrap();

            if can_num >= left_num && can_num <= right_num {
                ret.push(can_num);
            }
        }
        no_of_digits += 1;
        if no_of_digits > normalized_right.len() {
            break;
        }
    }
    Some(ret)
}

fn invalid_in_range(left_range: &str, right_range: &str) -> Option<Vec<u128>> {
    let mut ret: Vec<u128> = vec![];
    let max_digits = max(left_range.len(), right_range.len());
    for i in 0..max(1, max_digits / 2) {
        let r = invalid_in_range_with_digits(left_range, right_range, i + 1);
        match r {
            Some(v) => {
                for _v in v {
                    ret.push(_v);
                }
            }
            None => {}
        }
    }
    Some(ret)
}

pub fn calculate_invalid_in_range() {
    let file_lines = read_to_string("./puzzle_input_day2.txt").unwrap();
    let no_whitespace: String = file_lines.chars().filter(|c| !c.is_whitespace()).collect();
    let fs = no_whitespace.split(",");
    let mut ranges: Vec<(&str, &str)> = vec![];
    for f in fs {
        let gg = f.split("-").collect::<Vec<&str>>();
        ranges.push((gg[0], gg[1]));
    }
    let mut s: HashSet<u128> = HashSet::new();
    for (l, r) in ranges {
        print!("For range {l}..{r}\n");
        let inval = invalid_in_range(l, r);
        if let Some(v) = inval {
            for _v in v {
                print!("{_v}\n");
                s.insert(_v);
            }
        }
    }
    print!("***************************\n");
    let sum: u128 = s.iter().sum();
    print!("Sum = {sum}");
}
