use crate::utils;

pub fn brute_force(){
    let str_vec = utils::read_file("puzzle_input_day9.txt");
    let mut points:Vec<(i128,i128)> = Vec::new();
    for v in str_vec{
        let gg = v.split(",").collect::<Vec<&str>>();
        let x = gg[0].parse::<i128>().unwrap();
        let y=gg[1].parse::<i128>().unwrap();
        points.push((x,y));
    }
    let mut maxSurface:i128 =0;
    for i  in 0.. points.len(){
        for j in i .. points.len(){
            let a = points[i];
            let b = points[j];
            let surface = ((a.0-b.0).abs()+1)*((a.1-b.1).abs()+1);
            if surface> maxSurface{
                maxSurface = surface;
            }
        }
    }
    print!("Max surface = {maxSurface}\n");
}