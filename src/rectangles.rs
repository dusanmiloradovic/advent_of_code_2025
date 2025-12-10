use crate::utils;
use std::collections::BTreeMap;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum WalkDirection {
    Left,
    Right,
    Down,
    Up,
}

#[derive(Debug)]
struct ElfWalkStep {
    moves: usize,
    direction: WalkDirection,
}

type ElfWalk = Vec<ElfWalkStep>;
// This is a directed cyclic graph (ring) that gives the path

fn construct_elf_walk(initial_point: &(i128, i128), points: &[(i128, i128)]) -> ElfWalk {
    let mut vertical_direction = WalkDirection::Down;
    //once we react the bottom, we start going up
    let mut the_walk = ElfWalk::new();
    // The points are ordered by the row number of the input, and from the first point we are going right and down
    let mut point = initial_point;
    let mut current_direction = WalkDirection::Down; // the direction is the previous direction how we appear, 
    loop {
        if current_direction == WalkDirection::Up
            && point.0 == initial_point.0
            && point.1 == initial_point.1
        {
            break; //back to starting point, loop is complete
        }
        //break when the next point is the first point in the cycle TODO
        if current_direction == WalkDirection::Up || current_direction == WalkDirection::Down {
            let v = points
                .into_iter()
                .find(|p| p.1 == point.1 && p.0 != point.0)
                .unwrap(); // should not panic if everything good
            let dist = (point.0 - v.0).abs() as usize;
            let mut new_direction = WalkDirection::Right;
            if v.0 < point.0 {
                new_direction = WalkDirection::Left;
            }
            let step = ElfWalkStep {
                moves: dist,
                direction: new_direction,
            };
            point = v;
            the_walk.push(step);
            current_direction = new_direction;
        } else {
            if vertical_direction == WalkDirection::Down {
                match points.into_iter().find(|p| p.1 > point.1 && p.0 == point.0) {
                    None => {
                        vertical_direction = WalkDirection::Up;
                    }
                    Some(p) => {
                        let dist = (point.1 - p.1).abs() as usize;
                        point = p;
                        let step = ElfWalkStep {
                            moves: dist,
                            direction: vertical_direction,
                        };
                        the_walk.push(step);
                        current_direction = vertical_direction;
                    }
                }
            } else {
                match points.into_iter().find(|p| p.1 < point.1 && p.0 == point.0) {
                    None => {
                        break; // we reached the end and closed the curcuit
                    }
                    Some(p) => {
                        let dist = (point.1 - p.1).abs() as usize;
                        point = p;
                        let step = ElfWalkStep {
                            moves: dist,
                            direction: vertical_direction,
                        };
                        the_walk.push(step);
                        current_direction = vertical_direction;
                    }
                }
            }
        }
    }
    the_walk
}
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

fn find_the_upper_left_point(points: &[(i128, i128)]) -> ((i128, i128), i128) {
    //starting point and last row number
    let mut s: BTreeMap<i128, i128> = BTreeMap::new();
    for p in points {
        match s.get(&p.1) {
            None => {
                s.insert(p.1, p.0);
            }
            Some(x) => {
                s.insert(p.1, *x.min(&p.0));
            }
        }
    }
    let keys: Vec<&i128> = s.keys().collect();
    let y = keys[0];
    let x = s[y];
    let z = keys[keys.len() - 1];
    ((x, *y), *z)
}

fn rect_in_bounds(point_a: (i128, i128), point_b: (i128, i128), bounds: &[(i128, i128)]) -> i128 {
    let x_min = point_a.0.min(point_b.0);
    let x_max = point_a.0.max(point_b.0);
    let y_min = point_a.1.min(point_b.1);
    let y_max = point_a.1.max(point_b.1);
    for y in y_min..=y_max {
        let ex_bound = bounds[y as usize];
        if ex_bound.0 == -1 {
            return -1;
        }
        if !(x_min >= ex_bound.0 && x_max <= ex_bound.1) {
            return -1;
        }
    }
    (x_max - x_min + 1) * (y_max - y_min + 1)
}
pub fn do_the_elf_walk() {
    let mut max_area: i128 = 0;
    let points = get_input();
    let (point, last_row_number) = find_the_upper_left_point(&points);
    let elf_walk = construct_elf_walk(&point, &points);
    //print!("Debug elf walk {:#?}", elf_walk);
    let mut bounds: Vec<(i128, i128)> = vec![(-1, -1); (last_row_number + 1) as usize];
    let el0 = &elf_walk[0];
    let right_point = point.0 + el0.moves as i128;
    let ip = (point.0, right_point);
    let mut current_row = point.1 as usize;
    let mut current_column = right_point;
    bounds[point.1 as usize] = ip;
    for j in 1..elf_walk.len() {
        let e = &elf_walk[j];
        if e.direction == WalkDirection::Down {
            for j in 0..e.moves {
                current_row += 1;
                let mut ex_bound = bounds[current_row].clone();
                ex_bound.1 = current_column;
                bounds[current_row] = ex_bound;
                //paint the right side
            }
            continue;
        }
        if e.direction == WalkDirection::Right {
            current_column += e.moves as i128;
            // now if the current row is already painted to the right, don't do anything, just move the cursor
            let mut ex_bound = bounds[current_row].clone();
            if ex_bound.1 != -1 && ex_bound.1 >= current_column {
                continue;
            }
            ex_bound.1 = current_column;
            bounds[current_row] = ex_bound;
            continue;
        }
        if e.direction == WalkDirection::Left {
            current_column -= e.moves as i128;
            let mut ex_bound = bounds[current_row].clone();
            if ex_bound.0 != -1 && ex_bound.0 <= current_column {
                continue;
            }
            ex_bound.0 = current_column;
            bounds[current_row] = ex_bound;
        }
        if e.direction == WalkDirection::Up {
            for j in 0..e.moves {
                current_row -= 1;
                let mut ex_bound = bounds[current_row].clone();
                ex_bound.0 = current_column;
                bounds[current_row] = ex_bound;
            }
        }
    }
    //print!("Bounds\n{:?}", bounds);
    for i in 0..points.len() {
        for j in i..points.len() {
            let p1 = points[i];
            let p2 = points[j];
            let area = rect_in_bounds(p1, p2, &bounds);
            if area != -1 {
               // print!("Satisfied {:?}.. {:?}, {area}\n", p1, p2);
                if area > max_area {
                    max_area = area;
                }
            }
        }
    }
    print!("Max area = {max_area}\n");
}
