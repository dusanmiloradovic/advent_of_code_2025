use crate::utils;
use std::cmp::max;
use std::collections::BTreeMap;

fn get_puzzle_input() -> (Vec<(u64, u64)>, Vec<u64>) {
    let str_vec = utils::read_file("puzzle_input_day5.txt");
    let mut ranges: Vec<(u64, u64)> = Vec::new();
    let mut food_ids: Vec<u64> = Vec::new();
    let mut ranges_scan = true;
    for line in str_vec {
        let trimmed = line.trim();
        if trimmed == "" {
            ranges_scan = false;
            continue;
        }
        if ranges_scan {
            let gg = trimmed.split("-").collect::<Vec<&str>>();
            ranges.push((gg[0].parse::<u64>().unwrap(), gg[1].parse::<u64>().unwrap()))
        } else {
            food_ids.push(trimmed.parse::<u64>().unwrap());
        }
    }
    (ranges, food_ids)
}

pub fn count_fresh() {
    let (food_ranges, food_ids) = get_puzzle_input();
    let mut fresh_count: u32 = 0;
    for id in &food_ids {
        for (lower, higher) in &food_ranges {
            if id >= lower && id <= higher {
                fresh_count += 1;
                break;
            }
        }
    }
    print!("{fresh_count} fresh foods\n");
}

pub fn count_ranges() {
    let (food_ranges, _) = get_puzzle_input();
    let mut range_map: BTreeMap<u64, u64> = BTreeMap::new();
    for (lower, higher) in food_ranges {
        match range_map.get(&lower) {
            Some(val) => {
                if val < &higher {
                    range_map.insert(lower, higher);
                }
            }
            None => {
                range_map.insert(lower, higher);
            }
        }
    }
    let mut keys: Vec<u64> = range_map.keys().cloned().collect();
    for k in 1..keys.len() {
        let r_left_range = keys[k];
        // Get and clone the value, dropping the borrow immediately
        let r_right_range = match range_map.get(&r_left_range) {
            None => continue,  //could have been removed
            Some(&val) => val, // Copy the u64 value
        };

        for l in 0..k {
            let l_left_range = keys[l];
            if let Some(&l_right_range) = range_map.get(&l_left_range) {
                if l_right_range >= r_left_range {
                    let new_right_range = max(l_right_range, r_right_range);
                    range_map.insert(l_left_range, new_right_range);
                    range_map.remove(&r_left_range);
                }
            }
        }
    }
    let mut total_cnt: u64 = 0;
    keys = range_map.keys().cloned().collect();
    for k in keys {
        let v = range_map.get(&k).unwrap();
        print!("{k}..{v}\n");
        total_cnt += *v - k + 1;
    }
    print!("Total count: {total_cnt}\n");
}
