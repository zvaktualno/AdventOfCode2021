pub struct Board {
    board: Vec<Vec<u32>>,
    checked: Vec<Vec<bool>>,
    last_num: u32,
}

impl Board{
    pub fn new(b: &Vec<Vec<u32>>) -> Board
    {
        let board = Board {
            board: b.clone(),
            checked: vec![vec![false; b[0].len()]; b.len()],
            last_num: 0
        };
        //println!("Created board {}x{}", board.board.len(), board.board[0].len());
        return board
    }
    pub fn draw(&self) {
        for (ridx, row) in self.checked.iter().enumerate(){
            for (cidx, val) in row.iter().enumerate(){
                if *val {
                    print!("{}* ", self.board[ridx][cidx])
                }
                else
                {
                    print!("{}  ", self.board[ridx][cidx])
                }
            }
            println!("");
        }
        println!("");
        println!("");
    }

    pub fn winner(&self) -> bool {
        for row in &self.checked{
            if row.iter().filter(|x| **x == true).count() == row.len() {
                return  true;
            }
        }
        for col in 0..self.checked.len() {
            if self.checked.iter().filter(|x| x[col] == true).count() == self.checked.len(){
                return true;
            }
        }
        return false;
    }

    pub fn add_number(&mut self, num: u32) {
        for (ridx, row) in self.board.iter().enumerate(){
            for (cidx, val) in row.iter().enumerate(){
                if val.eq(&num) {
                    self.last_num = num;
                    self.checked[ridx][cidx] = true;
                }
            }
        }
    }

    pub fn score(&self) -> u32 {
        let mut sum: u32 = 0;
        for (ridx, row) in self.checked.iter().enumerate(){
            for (cidx, val) in row.iter().enumerate(){
                if !*val {
                    sum += self.board[ridx][cidx];
                }
            }
        }
        return sum * self.last_num;
    }
}