#[path = "./board.rs"]
mod board;

pub fn solve(lines: &Vec<String>) {
    
    let bingo_nums = lines[0].split(',').collect::<Vec<&str>>().iter().map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>();

    let mut parsed_board: Vec<Vec<u32>> = vec![];
    let mut boards: Vec<board::Board> = vec![];

    // Parse and create boards.
    for line in &lines[2..lines.len()]{
        if line == "" {
            boards.push(board::Board::new(&parsed_board));
            parsed_board = vec![];
        }
        else {
            let parsed_line: Vec<u32> = line.split_whitespace().map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>();
            parsed_board.push(parsed_line);
        }
    };
    boards.push(board::Board::new(&parsed_board));
    
    // Go over all numbers
    let mut res: u32 = 0;
    for bingo_num in bingo_nums{
        let mut boards_to_remove: Vec<usize> = vec![];

        // Add numbers to all boards, assuming only one can win at once
        for board in &mut boards{
            board.add_number(bingo_num);
        }

        // Save indexed of every winning board
        for (bidx, board) in boards.iter().enumerate(){
            if board.winner(){
                boards_to_remove.push(bidx);
            }
        }   

        // If one board left and it won, break
        if boards.len() == 1 && boards_to_remove.len() == 1 {
            res = boards[0].score();
            break;
        }

        // Reverse so correct indexes are removed
        for b in boards_to_remove.iter().rev(){
            boards.remove(*b);
        }
    }
    println!("Part two result {}", res);
}