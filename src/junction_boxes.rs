use crate::utils;
use std::collections::{BTreeMap, HashMap};

fn distance(x: (usize, usize, usize), y: (usize, usize, usize)) -> usize {
    let sq = (x.0 as i64 - y.0 as i64).pow(2)
        + (x.1 as i64 - y.1 as i64).pow(2)
        + (x.2 as i64 - y.2 as i64).pow(2);
    sq.isqrt() as usize
}

fn prepare_vectors() -> Vec<(usize, usize, usize)> {
    let str_vec = utils::read_file("puzzle_input_day8.txt");
    let mut ret: Vec<(usize, usize, usize)> = Vec::new();
    for s in str_vec {
        let t = s.split(",").collect::<Vec<&str>>();
        let v: (usize, usize, usize) = (
            t[0].parse::<usize>().unwrap(),
            t[1].parse::<usize>().unwrap(),
            t[2].parse::<usize>().unwrap(),
        );
        ret.push(v);
    }
    ret
}


fn new_way(vectors: Vec<(usize, usize, usize)>, top: usize) {
    let mut groups: Vec<Vec<usize>> = Vec::new();
    let mut distances: BTreeMap<usize, (usize, usize)> = BTreeMap::new();
    let mut group_mapping: HashMap<usize, usize> = HashMap::new();
    for i in 0..vectors.len() {
        for j in 0..vectors.len() {
            if i == j {
                continue;
            }
            let dist = distance(vectors[i], vectors[j]);
            distances.insert(dist, (i, j));
        }
    }
    let mut i: i32 = -1;
    for k in distances.keys() {
        i += 1;
        if (i >= top as i32) {
            break;
        }
        let (l, r) = distances[k];
        let mut l_exists = false;
        let mut r_exists = false;
        let mut lind: usize = 0;
        let mut rind: usize = 0;
        if let Some(x) = group_mapping.get(&l) {
            l_exists = true;
            lind = *x;
        }
        if let Some(y) = group_mapping.get(&r) {
            r_exists = true;
            rind = *y;
        }
        if l_exists && r_exists && lind == rind {
            continue;
        }
        if l_exists && r_exists {
            let r_group_ind = *group_mapping.get(&r).unwrap();
            let l_group_ind = *group_mapping.get(&l).unwrap();

            // Update all mappings from r_group to l_group
            let r_group_clone = groups[r_group_ind].clone();
            for rr in &r_group_clone {
                group_mapping.insert(*rr, l_group_ind);
            }

            // Merge the groups
            groups[l_group_ind].extend(r_group_clone);
            groups[r_group_ind].clear(); // Clear the old group

            continue;
        }
        if !l_exists && !r_exists {
            groups.push(vec![l, r]);
            let ind = groups.len() - 1;
            group_mapping.insert(l, ind);
            group_mapping.insert(r, ind);
            continue;
        }
        if l_exists && !r_exists {
            group_mapping.insert(r, lind);
            groups[lind].push(r);
            continue;
        }
        if r_exists && !l_exists {
            group_mapping.insert(l, rind);
            groups[rind].push(l);
            continue;
        }
    }
    print!("Groups {:?}\n", groups);
    let mut gcount = groups.into_iter().map(|g| g.len()).collect::<Vec<usize>>();
    gcount.sort();
    gcount.reverse();
    // print!("gcouunt {:?}\n",gcount);
    let rez = gcount[0] * gcount[1] * gcount[2];
    print!("rezult = {rez}\n");
}

pub fn get_areas_mul() {
    let vec = prepare_vectors();
    new_way(vec, 1000);
}
