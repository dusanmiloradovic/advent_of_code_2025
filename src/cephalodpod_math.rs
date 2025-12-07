use crate::utils;
fn smart_split(entry_str: &str) -> Vec<String> {
    let mut first_digit_passed = false;
    let mut ret: Vec<String> = Vec::new();
    let mut word = "".to_string();
    for i in 0..entry_str.len() {
        let c = &entry_str[i..i + 1];

        if c == " " && first_digit_passed {
            ret.push(word);
            first_digit_passed = false;
            word = "".to_string();
            continue;
        }
        if c != " " {
            first_digit_passed = true;
        }
        let ws = word.to_string() + c;
        word = ws;
        if c != " " && i == entry_str.len() - 1 {
            ret.push(word.clone()); // borrow checker woes
        }
    }
    ret
}
fn get_puzzle_input() -> (Vec<Vec<u128>>, Vec<String>, usize) {
    let str_vec = utils::read_file("puzzle_input_day6.txt");
    let mut matrix: Vec<Vec<u128>> = Vec::new();
    let mut width: usize = 0;
    for i in 0..str_vec.len() - 1 {
        let line = str_vec.get(i).unwrap();
        let vec_operands = smart_split(&line);
        width = vec_operands.len();
        let mut row: Vec<u128> = vec![0; vec_operands.len()];
        for j in 0..vec_operands.len() {
            row[j] = vec_operands[j].trim().parse::<u128>().unwrap();
        }
        matrix.push(row);
    }
    let last_line = str_vec.get(str_vec.len() - 1).unwrap();
    let op = smart_split(last_line)
        .into_iter()
        .map(|s| s.trim().to_string())
        .collect::<Vec<String>>();
    (matrix, op, width)
}

fn perform_ceph_oper(op: &str, inp: Vec<String>) -> u128 {
    print!("perform {op} \n");
    for i in &inp {
        print!("{i}\n")
    }
    print!("\n");
    let mut s: u128 = 0;
    if op == "*" {
        s = 1;
    }
    let str_len = inp[0].len(); // all have the same length

    for _j in 0..str_len {
        let j = str_len - _j - 1;
        let mut dig_str = "".to_string();
        for jj in &inp {
            let jj_dig = &jj[j..j + 1];
            if jj_dig == "" {
                continue;
            }
            dig_str += jj_dig;
        }

        if dig_str.trim()==""{
            break;
        }
        print!("s={dig_str} {op} {s}\n");
        let dig_num = dig_str.trim().parse::<u128>().unwrap();

        if op == "*" {
            s = s * dig_num;
        } else {
            s += dig_num;
        }
    }
    s
}

pub fn perform_ceph_math_puzzle() {
    let str_vec = utils::read_file("puzzle_input_day6.txt");

    let last_line = str_vec.get(str_vec.len() - 1).unwrap();
    let op = smart_split(last_line)
        .into_iter()
        .map(|s| s.trim().to_string())
        .collect::<Vec<String>>();
    let mut op_ind: usize = 0;
    let ln = op.len();
    //let mut words =vec!["".to_string();str_vec.len()-1];
    let mut prev_index: usize = 0;
    let mut ind: usize = 0;
    let mut ret: u128 = 0;
    loop {
        let mut all_blanks = true;
        let mut eol = false;
        for l in 0..str_vec.len() - 1 {
            let line = &str_vec[l];
            if ind == line.len() {
                eol = true;
                break;
            }
            let char = &line[ind..ind + 1];
            all_blanks = all_blanks && char == " ";
        }
        if all_blanks || eol {
            let mut words: Vec<String> = vec!["".to_string(); str_vec.len() - 1];
            for l in 0..str_vec.len() - 1 {
                let line = &str_vec[l];
                let word = line[prev_index..ind].to_string();
                words[l] = word;
            }
            ret += perform_ceph_oper(&op[op_ind], words);
            prev_index = ind;
            op_ind += 1;
        }
        if eol {
            break;
        }
        ind += 1;
    }
    print!("CephaloSum= {ret}\n");
}
fn perform_oper(oper: &str, a1: u128, a2: u128) -> u128 {
    if oper == "*" { a1 * a2 } else { a1 + a2 }
}
pub fn get_matrix_sum() {
    let (vec, operations, width) = get_puzzle_input();
    let mut sum: u128 = 0;
    for i in 0..width {
        print!("Column {i}\n");
        print!("*****************\n");
        let op = &operations[i];
        let mut s: u128 = 0;
        if op == "*" {
            s = 1;
        }
        for v in &vec {
            let opp = v[i];
            // print!("s = {s} {op} {opp}");
            s = perform_oper(op, s, opp);
            // print!("={s}\n");
        }
        sum += s;
    }
    print!("Sum={sum}\n");
}
