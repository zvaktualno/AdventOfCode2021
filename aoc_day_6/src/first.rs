use std::collections::HashMap;

fn fish_new_fish(ftimer: u32, days_left: u32) -> u64{
    if days_left == 0{
        return 1;
    }
    if ftimer >= days_left {
        return 1;    
    }
    let v2 = fish_new_fish(6, days_left - ftimer - 1);
    let v1 = fish_new_fish(8, days_left - ftimer - 1);
    return v1 + v2;
}

pub fn solve(lines: &Vec<String>) {
    let fishes = lines[0].split(",").map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>();
    let days = 80;

    let mut res: u64 = 0;
    let mut speedup_map: HashMap<u32, u64> = HashMap::new();
    for fish in &fishes{
        match speedup_map.get(fish){
            Some(x)=> {res += x;},
            None => {
                    let new_fish = fish_new_fish(*fish, days);
                    speedup_map.insert(*fish, new_fish);
                    res += new_fish;
                },
        }
    }
    
    println!("Part one result {}", res);
}