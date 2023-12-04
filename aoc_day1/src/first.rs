pub fn solve(lines: &Vec<String>) {
    let mut res: u32 = 0;
    let mut prev_val: u32 = lines[0].parse().unwrap();

    for line in &lines[1..lines.len()]{
        let new_val: u32 = line.parse().unwrap();
        if new_val > prev_val {
            res += 1;
        }
        prev_val = new_val;
    }
    
    println!("Part one result {}", res);
}