use std::fs::read_to_string;

fn invalid_in_normalized_range(left_range: &str, right_range: &str) -> Vec<u32> {
    let lr_num = left_range.parse::<u32>().unwrap();
    let rr_num = right_range.parse::<u32>().unwrap();
    // ranges have same length
    let mut ret: Vec<u32> = vec![];
    let mid_point = &left_range.len() / 2;
    let ll = &left_range[..mid_point];
    let rl = &right_range[..mid_point];
    let mut ml = ll.parse::<u32>().unwrap();
    let mr = rl.parse::<u32>().unwrap();
    while ml <= mr {
        let candidate = format!("{}{}", &ml, &ml);
        let cn = candidate.parse::<u32>().unwrap();
        if (&cn >= &lr_num && &cn <= &rr_num) {
            ret.push(cn);
        }
        ml = ml + 1;
    }
    ret
}

fn normalize_range(left_range: &str, right_range: &str) -> Option<(String, String)> {
    let mut lr = left_range.to_string();
    let mut rr = right_range.to_string();
    let lr_odd_digits = lr.len() % 2 != 0;
    let rr_odd_digits = rr.len() % 2 != 0;
    if lr_odd_digits && rr_odd_digits {
        return None;
    }

    if lr_odd_digits {
        let base: u32 = 10;
        let v = base.pow(lr.len() as u32);
        lr = v.to_string();
    }
    if rr_odd_digits {
        let base: u32 = 10;
        let v = base.pow(rr.len() as u32 + 1) - 1;
        rr = v.to_string();
    }
    Some((lr, rr))
}

fn calculate_invalid_in_ranges(ranges: &[(&str, &str)]) -> Vec<u32> {
    let mut ret: Vec<u32> = vec![];
    for (l, r) in ranges {
        let nr = normalize_range(l, r);
        match nr {
            Some((lr, rr)) => {
                let v = invalid_in_normalized_range(&lr, &rr);
                for _v in v {
                    ret.push(_v);
                }
            }
            None => {}
        }
    }
    ret
}

pub fn calculate_invalid_in_range() {
    let file_lines = read_to_string("./puzzle_input_day2.txt").unwrap();
    let fs = file_lines.split(",");
    let mut ranges: Vec<(&str, &str)> = vec![];
    for f in fs {
        let gg = f.split("-").collect::<Vec<&str>>();
        ranges.push((gg[0], gg[1]));
    }

    let v = calculate_invalid_in_ranges(&ranges);
    for _v in v {
        print!("{_v}\n");
    }
}
