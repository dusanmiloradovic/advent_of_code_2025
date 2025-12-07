use crate::utils;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

struct BTree {
    node: usize,
    left: RefCell<Option<Rc<BTree>>>,
    right: RefCell<Option<Rc<BTree>>>,
}
fn get_beams(previous_beams: &[usize], splitters: &[usize]) -> (usize, Vec<usize>) {
    //returns the split count and new beams
    let mut split_count: usize = 0;
    let mut ret: Vec<usize> = Vec::new();
    if splitters.len() == 0 {
        return (0, previous_beams.to_vec());
    }
    let mut j = 0; // index for splitters
    let mut k = 0; // index for beams
    let splitters_len = splitters.len();
    let beams_len = previous_beams.len();
    loop {
        if k == beams_len {
            break;
        }
        if j == splitters_len {
            // first copy the rest of beams
            for i in k..beams_len {
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
        if beam > splitter {
            j += 1;
            ret.push(beam);
        }
    }
    (split_count, ret)
}


fn get_splitters(line: &str) -> Vec<usize> {
    let mut ret: Vec<usize> = Vec::new();
    for i in 0..line.len() {
        if &line[i..i + 1] == "^" {
            ret.push(i);
        }
    }
    ret
}

fn construct_binary_tree() -> Rc<BTree> {
    let str_vec = utils::read_file("puzzle_input_day7_test.txt");
    let first_line = str_vec[0].as_str();
    let ind = first_line.find("S").unwrap();
    let mut lookup_map: HashMap<usize, Rc<BTree>> = HashMap::new();
    let mut pending: Vec<usize> = Vec::new();

    let btree: BTree = BTree {
        node: ind,
        left: RefCell::new(None),
        right: RefCell::new(None),
    };
    let ret = Rc::new(btree);
    let ret1 = Rc::clone(&ret);
    lookup_map.insert(ind, ret);
    pending.push(ind);

    for i in 1..str_vec.len() {
        print!("Processing line {i}\n");
        let splitters = get_splitters(&str_vec[i]);
        let pen_len = pending.len();
        let splitters_len = splitters.len();
        let mut pending_counter: usize = 0;
        let mut splitter_counter: usize = 0;
        let mut pen_removal:Vec<usize>=Vec::new();
        loop {
            if pending_counter == pen_len || splitters_len == 0 || splitter_counter == splitters_len{
                break;
            }
            let pen = pending[pending_counter];
            let s = &splitters[splitter_counter];
            if pen < *s {
                splitter_counter += 1;
                continue;
            }

            if pen == *s {
                let ex = Rc::clone(lookup_map.get(&s).unwrap());

                let left = Rc::new(BTree {
                    node: s - 1,
                    left: RefCell::new(None),
                    right: RefCell::new(None),
                });
                let right = Rc::new(BTree {
                    node: s + 1,
                    left: RefCell::new(None),
                    right: RefCell::new(None),
                });
                lookup_map.insert(s - 1, Rc::clone(&left));
                lookup_map.insert(s + 1, Rc::clone(&right));
                //pending.remove(pending_counter);
                pen_removal.push(pen);
                *ex.left.borrow_mut() = Some(Rc::clone(&left));
                *ex.right.borrow_mut() = Some(Rc::clone(&right));
                pending.push(s - 1);
                pending.push(s + 1);
                pending_counter += 1;
                splitter_counter += 1;
                continue;
            }
            if pen > *s {
                pending_counter += 1;
                splitter_counter += 1;
            }
        }
        for p in pen_removal{
            pending.retain(|value| *value != p);
        }
    }
    ret1
}
pub fn get_beam_splits() {
    let mut split_cnt: usize = 0;
    let str_vec = utils::read_file("puzzle_input_day7_test.txt");
    let first_line = str_vec[0].as_str();
    let ind = first_line.find("S").unwrap();
    let mut beams = vec![ind];
    let mut cnt = 0;
    for i in 1..str_vec.len() {
        let splitters = get_splitters(&str_vec[i]);
        (cnt, beams) = get_beams(&beams, &splitters);
        print!("beams {:?}\n", beams);
        split_cnt += cnt;
    }
    print!("Beam split count={split_cnt}");
}

pub fn get_count_of_tree() {
    let btree = construct_binary_tree();
    print!("dusan\n");
}
