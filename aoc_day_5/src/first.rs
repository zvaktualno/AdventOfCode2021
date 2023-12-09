use std::cmp::max;

struct Point {
    x: i32,
    y: i32,
}

fn draw_line(map: &mut Vec<Vec<i32>>, p1: &Point, p2: &Point)
{
    if p1.x == p2.x || p1.y == p2.y{
        let steps = max((p1.x - p2.x).abs(), (p1.y - p2.y).abs());
        let x_step: i32 = (p2.x - p1.x)/steps;
        let y_step = (p2.y - p1.y)/steps;
        for n in 0..steps+1{
            let new_y =p1.y + n * y_step;
            let new_x =p1.x + n * x_step;
            map[new_y as usize][new_x as usize] += 1;
        }
    }
}

pub fn solve(lines: &Vec<String>) {
    let mut res: i32 = 0;
    let mut map: Vec<Vec<i32>> = vec![vec![0; 1000];1000];
    for line in lines{
        let parsed_line: Vec<i32> = line.replace(" -> ", ",").split(",").map(|x| x.parse::<i32>().unwrap()).collect();
        let start:Point = Point {x: parsed_line[0], y: parsed_line[1]};
        let end:Point = Point {x: parsed_line[2], y: parsed_line[3]};
        draw_line(&mut map, &start, &end);
    }

    for c in &map{
        res += c.into_iter().filter(|x| **x > 1).count() as i32;
    }
    println!("Part one result {}", res);
}