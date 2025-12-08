use crate::utils;
use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::rc::Rc;

#[derive(Clone, Debug)]
struct BTree {
    node: usize,
    left: RefCell<Option<Rc<BTree>>>,
    right: RefCell<Option<Rc<BTree>>>,
}
fn get_beams(previous_beams: &[usize], splitters: &[usize], mut paths: Vec<Vec<usize>>) -> (usize, Vec<usize>, Vec<Vec<usize>>) {
    let mut split_count: usize = 0;
    let mut ret: Vec<usize> = Vec::new();
    
    if splitters.is_empty() {
        return (0, previous_beams.to_vec(), paths);
    }
    
    let mut j = 0; // index for splitters
    let mut k = 0; // index for beams
    let splitters_len = splitters.len();
    let beams_len = previous_beams.len();
    
    while k < beams_len {
        if j == splitters_len {
            // Copy the rest of beams
            ret.extend_from_slice(&previous_beams[k..]);
            break;
        }
        
        let splitter = splitters[j];
        let beam = previous_beams[k];
        
        if beam < splitter {
            ret.push(beam);
            k += 1;
        } else if beam == splitter {
            split_count += 1;
            ret.push(beam - 1);
            ret.push(beam + 1);
            k += 1;
            j += 1;
            
            // Update paths in-place
            let mut indices_to_split = Vec::new();
            for (idx, path) in paths.iter().enumerate() {
                if *path.last().unwrap() == beam {
                    indices_to_split.push(idx);
                }
            }
            
            // Process splits from back to front to maintain valid indices
            for &idx in indices_to_split.iter().rev() {
                let mut path = paths.swap_remove(idx);
                let mut path2 = path.clone();
                path.push(beam - 1);
                path2.push(beam + 1);
                paths.push(path);
                paths.push(path2);
            }
        } else { // beam > splitter
            j += 1;
            // Don't increment k, re-check this beam with next splitter
            continue;
        }
    }
    
    (split_count, ret, paths)
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
    let mut gl_splitter = 0;
    for i in 1..str_vec.len() {
        //  print!("Processing line {i}\n");
        print!("Pending {:?}\n", pending);
        let splitters = get_splitters(&str_vec[i]);
        print!("Splitters {:?}\n", splitters);
        let pen_len = pending.len();
        let splitters_len = splitters.len();
        let mut pending_counter: usize = 0;
        let mut splitter_counter: usize = 0;
        let mut pen_removal: Vec<usize> = Vec::new();
        loop {
            if pending_counter == pen_len || splitters_len == 0 || splitter_counter == splitters_len
            {
                break;
            }
            let pen = pending[pending_counter];
            let s = &splitters[splitter_counter];
            if pen < *s {
                pending_counter += 1;
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
                // print!("Constructed {:?}\n", ex);
                let sp = s - 1;
                if !pending.contains(&sp) {
                    pending.push(sp)
                };
                let sn = s + 1;
                if !pending.contains(&sn) {
                    pending.push(sn);
                }
                pending_counter += 1;
                splitter_counter += 1;
                gl_splitter += 2;
                continue;
            }
            if pen > *s {
                pending_counter += 1;
            }
        }
        for p in pen_removal {
            pending.retain(|value| *value != p);
        }
    }
    print!("Global splitter {gl_splitter}");
    ret1
}
pub fn get_beam_splits() {
    let mut split_cnt: usize = 0;
    let str_vec = utils::read_file("puzzle_input_day7.txt");
    let first_line = str_vec[0].as_str();
    let ind = first_line.find("S").unwrap();
    let mut beams = vec![ind];
    let mut cnt = 0;
    let mut prev_splitters :Vec<usize> = Vec::new();
    let mut paths :Vec<Vec<usize>> = vec![beams.clone()];


    for i in 1..str_vec.len() {
        print!("Starting line {i}");
        let splitters = get_splitters(&str_vec[i]);
        prev_splitters = splitters.clone();
        let old_beams = beams.clone();
        (cnt, beams,paths) = get_beams(&beams, &splitters,paths);
       // print!("beams {:?}\n", beams);
        split_cnt += cnt;

    }
    //print!("paths {:?}\n", paths);
    print!("Beam split count={split_cnt}");
    print!("path count {:?}\n", paths.len());
}

fn bfs_count(bt: Rc<BTree>) {
    print!("Tree {:?}", bt);
    let mut ret: Vec<String> = Vec::new();
    let mut queue: VecDeque<(Rc<BTree>, String)> = VecDeque::new();
    queue.push_back((bt.clone(), bt.node.to_string()));
    loop {
        if queue.len() == 0 {
            break;
        }
        let (tree, ex_str) = queue.pop_front().unwrap();
        let ex_str_cp = ex_str.clone();
        let ex_str_cp2 = ex_str.clone(); // this is lame, practice more borrow checker
        let mut left_empty = false;
        let mut right_empty = false;

        if let Some(b) = tree.left.borrow().clone() {
            let p = ex_str + "->L " + &b.node.to_string();
            queue.push_back((b.clone(), p));
        } else {
            //  ret.push(ex_str); //reach to the end;
            left_empty = true;
        }
        if let Some(b) = tree.right.borrow().clone() {
            let p = ex_str_cp + "->R " + &b.node.to_string();
            queue.push_back((b.clone(), p));
        } else {
            // ret.push(ex_str_cp); //reach to the end;
            right_empty = true;
        }
        if left_empty && right_empty {
            ret.push(ex_str_cp2);
        }
    }
    let lren = ret.len();
    print!("@@@{lren} \n");
    print!("ret {:?}\n", ret);
}

pub fn get_count_of_tree() {
    let btree = construct_binary_tree();
    bfs_count(btree.clone());
}
