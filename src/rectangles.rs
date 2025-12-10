use crate::utils;
use std::collections::BTreeMap;

pub fn brute_force() {
    let points = get_input();
    let mut maxSurface: i128 = 0;
    for i in 0..points.len() {
        for j in i..points.len() {
            let a = points[i];
            let b = points[j];
            let surface = ((a.0 - b.0).abs() + 1) * ((a.1 - b.1).abs() + 1);
            if surface > maxSurface {
                maxSurface = surface;
            }
        }
    }
    print!("Max surface = {maxSurface}\n");
}

fn get_input() -> Vec<(i128, i128)> {
    let str_vec = utils::read_file("puzzle_input_day9_test.txt");
    let mut points: Vec<(i128, i128)> = Vec::new();
    for v in str_vec {
        let gg = v.split(",").collect::<Vec<&str>>();
        let x = gg[0].parse::<i128>().unwrap();
        let y = gg[1].parse::<i128>().unwrap();
        points.push((x, y));
    }
    points
}
fn find_intersection(vec1: &[i128], vec2: &[i128]) -> Vec<i128> {
    let mut ret: Vec<i128> = vec![-1, -1];
    if vec1[0] == -1 || vec1[1] ==-1 || vec2[0] ==-1 || vec2[1] == -1 {
        return ret;
    }
    let maxl = vec1[0].max(vec2[0]);
    let minr = vec1[1].min(vec2[1]);
    if (maxl <= minr) {
        ret = vec![maxl, minr];
    }
    ret
}
pub fn green_red() {
    let dots = get_input();
    let mut map: BTreeMap<i128, Vec<i128>> = BTreeMap::new();
    for d in dots {
        let y = d.1;
        match map.get_mut(&y) {
            None => {
                map.insert(y, vec![d.0]);
            }
            Some(v) => {
                // based on the input its always sorted, and it will have only 2 items in vector
                v.push(d.0);
            }
        }
    }
    print!("map {:#?}", map);
    let mut max_rectangle_area: i128 = 0;
    let keys_vec: Vec<i128> = map.keys().copied().collect();
    for upper_y_ind in 0..keys_vec.len() - 1 {
        let upper_y = keys_vec[upper_y_ind];
        let upper_line = map.get(&upper_y).unwrap();
        let mut intersection = upper_line.clone();
        for lower_y_ind in upper_y_ind + 1..keys_vec.len() {
            let lower_y = keys_vec[lower_y_ind];
            let lower_line = map.get(&(upper_y as i128)).unwrap();
            intersection = find_intersection(&intersection, lower_line);
            if lower_y == upper_y + 1 && intersection[0] != -1 {
                // for neighbouring line, intersection is the x width;
                let w = intersection[1] - intersection[0] + 1;
                let h = lower_y - upper_y + 1;
                let area = w * h;
                if max_rectangle_area < area {
                    max_rectangle_area = area;
                }
            }
            if lower_line[0] >= intersection[0]
                && upper_line[0] >= intersection[0]
                && lower_line[1] <= intersection[1]
                && upper_line[1] <= intersection[1]
            {
                let w = intersection[1] - intersection[0] + 1;
                let h = lower_y - upper_y + 1;
                let area = w * h;
                if max_rectangle_area < area {
                    max_rectangle_area = area;
                }
            }
        }
    }
    print!("Max rectangle area {max_rectangle_area}");
}
