pub fn solve(lines: &Vec<String>) {

    let mut hpos:u32 = 0;
    let mut vpos:u32 = 0;
    let mut aim: u32 = 0;

    for line in lines{
        let split_line = line.split(" ").collect::<Vec<&str>>();
        let amount = split_line[1].parse::<u32>().unwrap();
        match split_line[0]{
            "forward" => {hpos += amount; vpos += aim * amount},
            "down"=> {aim += amount},
            "up" => {aim -= amount},
            _ => panic!(),
        }
    }
    let res = hpos * vpos;
    println!("Part two result {}", res);
}