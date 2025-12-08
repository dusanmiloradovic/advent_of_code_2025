use std::cmp::Ordering;
use crate::utils;

fn distance(x:(f64,f64,f64), y:(f64,f64,f64))->f64{
    let sq = (x.0-y.0).powf(2f64)+(x.1-y.1).powf(2f64)+(x.2-y.2).powf(2f64);
    (sq as f64).sqrt()
}
#[derive(Debug)]
struct Line{
    dota:(f64,f64,f64),
    dotb:(f64,f64,f64),
}
impl PartialEq for Line {
    fn eq(&self, other: &Self) -> bool {
        self.dota == other.dota && self.dotb==other.dotb
    }
}

impl Eq for Line {}

impl PartialOrd for Line {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let val1 = distance(self.dota,self.dotb);
        let val2 = distance(other.dota,other.dotb);
        val1.partial_cmp(&val2)
    }
}

impl Ord for Line{
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}




fn prepare_vectors() -> Vec<(f64, f64, f64)> {
    let str_vec = utils::read_file("puzzle_input_day8_test.txt");
    let mut ret: Vec<(f64, f64, f64)> = Vec::new();
    for s in str_vec {
        let t = s.split(",").collect::<Vec<&str>>();
        let v: (f64, f64, f64) = (t[0].parse::<f64>().unwrap(), t[1].parse::<f64>().unwrap(), t[2].parse::<f64>().unwrap());
        ret.push(v);
    }
    ret
}
fn calculate(vectors:Vec<(f64,f64,f64)>, top:usize){
    let mut lista: Vec<Line> = Vec::new();
    for i in 0..vectors.len(){
        for j in i+1..vectors.len(){
            let line = Line{
                dota: vectors[i],
                dotb: vectors[j],
            };
            lista.push(line);
        }
    }
    lista.sort();
    for i in 0..top{
        let line = &lista[i];
        print!("Line {:?}\n",line);
    }
}

pub fn get_areas_mul(){
    let vec = prepare_vectors();
    calculate(vec,10);
}