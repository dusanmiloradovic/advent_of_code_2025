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
        if current_direction == WalkDirection::Up && point.0 == initial_point.0 && point.1 == initial_point.1{
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

fn find_the_upper_left_point(points: &[(i128, i128)]) -> (i128, i128) {
    let mut s:BTreeMap<i128,i128>=BTreeMap::new();
    for p in points{
        match s.get(&p.1){
            None=>{
                s.insert(p.1,p.0);
            },
            Some(x)=>{
                s.insert(p.1,*x.min(&p.0));
            }
        }
    }
    let keys:Vec<&i128> =s.keys().collect();
    let y = keys[0];
    let x = s[y];
    (x,*y)
}
pub fn do_the_elf_walk() {
    let points = get_input();
    let point = find_the_upper_left_point(&points);
    let elfWalk = construct_elf_walk(&point, &points);
    print!("Debug elf walk {:#?}",elfWalk);

}
