use crate::utils;

fn get_beams( previous_beams: &[usize], splitters: &[usize]) -> (usize, Vec<usize>) {
    //returns the split count and new beams
    let mut split_count: usize = 0;
    let mut ret: Vec<usize> = Vec::new();
    if splitters.len() == 0 {
        return (0, previous_beams.to_vec());
    }
    let mut j = 0; // index for splitters
    let mut k = 0; // index for beams
    let splitters_len = splitters.len();
    let beams_len=previous_beams.len();
    loop {
        if k == beams_len {
            break;
        }
        if j == splitters_len{
            // first copy the rest of beams
            for i in k..beams_len{
                ret.push(previous_beams[k]);
            }
            break;
        }
        let splitter = splitters[j];
        let beam = previous_beams[k];
        if beam < splitter {
            ret.push(beam);
            k += 1;
            continue;
        }
        if beam == splitter {
            // will not check for edge cases for now
            split_count += 1;
            ret.push(beam - 1);
            ret.push(beam + 1);
            k += 1;
            j += 1;
            continue;
        }
        if beam>splitter{
            j +=1;
            ret.push(beam);
        }
    }
    (split_count, ret)
}

fn get_splitters(line:&str)->Vec<usize>{
    let mut ret:Vec<usize> = Vec::new();
    for i in 0..line.len(){
        if &line[i..i+1] == "^"{
            ret.push(i);
        }
    }
    ret
}

pub fn get_beam_splits(){
    let mut split_cnt :usize = 0;
    let str_vec = utils::read_file("puzzle_input_day7.txt");
    let first_line=str_vec[0].as_str();
    let ind = first_line.find("S").unwrap();
    let mut beams=vec![ind];
    let mut cnt = 0;
    for i in 1..str_vec.len(){
        let splitters = get_splitters(&str_vec[i]);
        (cnt,beams) = get_beams(&beams, &splitters);
        split_cnt+=cnt;
    }
    print!("Beam split count={split_cnt}");
}
