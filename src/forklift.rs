use std::fs::read_to_string;

fn get_surrounding_count(v: &mut Vec<Vec<u8>>, i: usize, j: usize) -> u8 {
    let mut ret: u8 = 0;
    if i > 0 {
        let prev_row = &v[i - 1];
        if j > 0 {
            ret += prev_row[j - 1];
        }
        if j < &prev_row.len() - 1 {
            ret += prev_row[j + 1];
        }
        ret += prev_row[j];
    }
    if i < &v.len() - 1 {
        let next_row = &v[i + 1];
        if j > 0 {
            ret += next_row[j - 1];
        }
        if j < &next_row.len() - 1 {
            ret += next_row[j + 1];
        }
        ret += next_row[j];
    }
    let row = &v[i];
    if j > 0 {
        ret += row[j - 1];
    }
    if j < &row.len() - 1 {
        ret += row[j + 1];
    }
    ret
}

fn count_accessible(places: &mut Vec<Vec<u8>>) -> u32 {
    let mut ret: u32 = 0;
    let mut dels: Vec<(usize, usize)> = Vec::new();
    let row_len = places.len();
    for i in 0..row_len {
        //let row = &places[i];
        let col_len = places[i].len();
        for j in 0..col_len {
            let cnt = get_surrounding_count(places, i, j);
            let val = places[i][j];
            // print!("{val} ... {i},{j}={cnt}\n");
            if cnt < 4 && places[i][j] == 1 {
                //print!("{i},{j}\n");
                ret += 1;
                dels.push((i, j));
            }
        }
    }
    for (i, j) in dels {
        places[i][j] = 0;
    }
    ret
}

fn parse_puzzle_input(input_lines: Vec<String>) -> Vec<Vec<u8>> {
    let input_size = input_lines.len();
    let mut ret: Vec<Vec<u8>> = Vec::new();
    for i in 0..input_size {
        let line = &input_lines[i];
        let mut line_vec: Vec<u8> = vec![0; line.len()];
        for j in 0..line.len() {
            let str = &line[j..j + 1];
            if str == "@" {
                line_vec[j] = 1
            }
        }
        ret.push(line_vec);
    }
    ret
}

pub fn find_roll_count() {
    let mut arr = get_puzzle_input_arr();
    let cnt = count_accessible(&mut arr);
    print!("{cnt} roll counts\n");
}

fn get_puzzle_input_arr() -> Vec<Vec<u8>> {
    let lines: Vec<String> = read_to_string("puzzle_input_day4.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();
    let arr = parse_puzzle_input(lines);
    arr
}

pub fn find_total_roll_count() {
    let mut total_cnt: u32 = 0;
    let mut arr = get_puzzle_input_arr();
    loop {
        let cnt = count_accessible(&mut arr);
        if cnt == 0 {
            break;
        }
        total_cnt += cnt;
    }
    print!("{total_cnt} total roll count\n");
}
