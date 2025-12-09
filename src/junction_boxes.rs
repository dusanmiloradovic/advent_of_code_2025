use crate::utils;
use std::cmp::Ordering;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::fmt;

fn distance(x: (usize, usize, usize), y: (usize, usize, usize)) -> usize {
    let sq = (x.0 as i64 - y.0 as i64).pow(2)
        + (x.1 as i64 - y.1 as i64).pow(2)
        + (x.2 as i64 - y.2 as i64).pow(2);
    sq.isqrt() as usize
}
#[derive(Hash, Clone)]
struct Line {
    dota: (usize, usize, usize),
    dotb: (usize, usize, usize),
}
impl PartialEq for Line {
    fn eq(&self, other: &Self) -> bool {
        self.dota == other.dota && self.dotb == other.dotb
    }
}

impl Eq for Line {}

impl PartialOrd for Line {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let val1 = distance(self.dota, self.dotb);
        let val2 = distance(other.dota, other.dotb);
        val1.partial_cmp(&val2)
    }
}

impl Ord for Line {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl fmt::Debug for Line {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let a0 = self.dota.0;
        let a1 = self.dota.1;
        let a2 = self.dota.2;
        let b0 = self.dotb.0;
        let b1 = self.dotb.1;
        let b2 = self.dotb.2;
        let d = distance(self.dota, self.dotb);
        write!(f, "Line ({a0},{a1},{a2})-({b0},{b1},{b2}) === {d}")
    }
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

fn calculate(vectors: Vec<(usize, usize, usize)>, top: usize) {
    let mut groups: Vec<u32> = Vec::new();
    let mut dot_group_mapping: HashMap<(usize, usize, usize), usize> = HashMap::new();
    let mut reverse_group_lookup: HashMap<usize, Vec<(usize, usize, usize)>> = HashMap::new();
    let mut lista: Vec<Line> = Vec::new();
    for i in 0..vectors.len() {
        for j in i + 1..vectors.len() {
            if i == j {
                continue;
            }
            let line = Line {
                dota: vectors[i],
                dotb: vectors[j],
            };
            lista.push(line);
        }
    }
    lista.sort();
    print!("Lista {:#?}", lista);
    let mut cnt = 0;
    // let mut i = 0; // restarting loop

    for i in 0..lista.len() {
        //let kons = count_connections(&groups);
        if cnt >= top {
            break;
        }
        print!("cycle = ${i}, cnt=${cnt}");
        // print!("groups={:?}\n", groups);

        let line = &lista[i];
        let mut exist_a = false;
        let mut exist_b = false;
        let mut dot_pos_a: usize = 0;
        let mut dot_pos_b: usize = 0;
        if let Some(dot) = dot_group_mapping.get(&line.dota) {
            dot_pos_a = *dot;
            exist_a = true;
        }
        if let Some(dot) = dot_group_mapping.get(&line.dotb) {
            dot_pos_b = *dot;
            exist_b = true;
            // groups[*dot] += 1;
            // dot_group_mapping.insert(approx(line.dota),*dot);
            // TODO this is random, I will check on the data is it working, or there is some hidden logic
            // actually this is ok , since if both dots already belong to the same set nothing should be done
        }
        if exist_a && exist_b && dot_pos_a == dot_pos_b {
            print!("line already exist, skip\n");
            continue;
        }
        if exist_a && exist_b {
            groups[dot_pos_a] += groups[dot_pos_b];
            groups[dot_pos_b] = 0;
            if let Some(vec) = reverse_group_lookup.get(&dot_pos_b) {
                for v in vec {
                    dot_group_mapping.insert(*v, dot_pos_a);
                }
            }
            reverse_group_lookup.remove(&dot_pos_b);
            cnt += 1;
            continue;
        }
        if exist_a {
            groups[dot_pos_a] += 1;
            cnt += 1;
            dot_group_mapping.insert(line.dotb, dot_pos_a);
            add_to_reverse_group(&mut reverse_group_lookup, &line.dotb, &dot_pos_a);
            print!("dota already exist inserting dotb\n");
            continue;
        }
        if exist_b {
            groups[dot_pos_b] += 1;
            cnt += 1;
            dot_group_mapping.insert(line.dota, dot_pos_b);
            add_to_reverse_group(&mut reverse_group_lookup, &line.dota, &dot_pos_b);
            print!("dotb already exist inserting dota\n");
            continue;
        }

        cnt += 1;
        groups.push(2);
        let pos = groups.len() - 1;
        dot_group_mapping.insert(line.dota, pos);
        dot_group_mapping.insert(line.dotb, pos);
        add_to_reverse_group(&mut reverse_group_lookup, &line.dota, &pos);
        add_to_reverse_group(&mut reverse_group_lookup, &line.dotb, &pos);
        print!("inserting both dota and dotb\n");
    }
    groups.sort();
    print!("Groups:{:?}\n", groups);
    let gg = count_connections(&groups);
    print!("Konneckija {gg}");
    // print!("dot group mapping {:#?}\n", dot_group_mapping);
}

fn count_connections(groups: &Vec<u32>) -> usize {
    let mut gg = 0;
    for g in groups {
        if *g != 0 {
            gg += g - 1;
        }
    }
    gg as usize
}

fn add_to_reverse_group(
    reverse_group_lookup: &mut HashMap<usize, Vec<(usize, usize, usize)>>,
    dot: &(usize, usize, usize),
    pos: &usize,
) {
    match reverse_group_lookup.get(&pos) {
        None => {
            reverse_group_lookup.insert(*pos, vec![*dot]);
        }
        Some(v) => {
            let mut vcl = v.clone();
            vcl.push(*dot);
            reverse_group_lookup.insert(*pos, vcl);
        }
    }
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
