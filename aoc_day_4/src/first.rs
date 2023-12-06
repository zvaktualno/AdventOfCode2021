#[path = "./board.rs"]
mod board;

pub fn solve(lines: &Vec<String>) {
    
    let bingo_nums = lines[0].split(',').collect::<Vec<&str>>().iter().map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>();

    let mut parsed_board: Vec<Vec<u32>> = vec![];
    let mut boards: Vec<board::Board> = vec![];

    // Parse and create boards.
    for line in &lines[2..lines.len()]{
        if line.is_empty(){
            boards.push(board::Board::new(&parsed_board));
            parsed_board = vec![];
        }
        else {
            let parsed_line: Vec<u32> = line.split_whitespace().map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>();
            parsed_board.push(parsed_line);
        }
    };

    let mut res: u32 = 0;
    'outer: for bingo_num in bingo_nums{
        for board in &mut boards{
            board.add_number(bingo_num);
            if board.winner(){
                res = board.score();
                break 'outer;
            }
        }
    }
    println!("Part one result {}", res);
}