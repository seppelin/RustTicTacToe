use crate::game::{BoardPiece, CheckResult};

use super::{AlgoMove, AlgoCheckRes};

pub struct AlgoBitBoard(u64, u16);

impl AlgoBitBoard {
    pub fn empty() -> AlgoBoard{
    	AlgoBoard(0, 0)
    }

    pub fn from_board(board: &[[Vec<BoardPiece>; 3]; 3], p_available: &[[usize; 3]; 2]) -> Self{
        let mut a_board = AlgoBoard::empty();
        for w in 0..3{
            for h in 0..3{
                let loc = (w*3+h)*6;
                for piece in &board[w][h]{
                    let pos = (piece.piece.as_usize()*2+loc) as u8;
                    if piece.sign{
                        a_board.set(pos + 1);
                    }
                    a_board.set(pos);
                }
            }
        }
        for sign in 0..2{
            for size in 0..3{
                match p_available[sign][size] {
                    0 => (),
                    1 => a_board.set_p((sign*6+size*2) as u8),
                    2 => {
                        a_board.set_p((sign*6+size*2) as u8);
                        a_board.set_p((sign*6+size*2+1) as u8)
                    }
                    _ => panic!("wrong p_available")
                }
            }
        }
        a_board
    }

    pub fn get_raw(&self) -> &(u64, u16){
        &(self.0, self.1)
    }

    pub fn get_move_options(&self, sign: bool) -> Vec<(AlgoMove, u8)>{
        let options = Vec::new();
        for field in 0..9{
            let size = if self.get(field*6+4){
                if self.get(field*6+5) == sign{
                    2
                }else{
                    continue;
                }
            }else if self.get(field*6+2) {
                if self.get(field*6+3) == sign{
                    1
                }else{
                    continue;
                }
            }
            else if self.get(field*6){
                if self.get(field*6+1) == sign{
                    0
                }else{
                    continue;
                }
            }
            else {
                continue;
            };
            
        }
        options
    }

    pub fn do_move_false(&mut self, from: &AlgoMove, to: u8){
        match from {
            AlgoMove::NewP(size) => {
                self.set(to + *size*2);
                self.unset(to + *size*2 + 1);
            }
            AlgoMove::MoveP(index, size) => {
                self.unset(index + size*2); // disable the from field

                self.set(to + size*2); // enable the to field
                self.unset(to + size*2 + 1);
            }
        }
    }

    pub fn do_move_true(&mut self, from: &AlgoMove, to: u8){
        match from {
            AlgoMove::NewP(size) => {
                self.set(to + *size*2);
                self.set(to + *size*2 + 1);
            }
            AlgoMove::MoveP(index, size) => {
                self.unset(index + size*2); // disable the from field

                self.set(to + size*2); // enable the to field
                self.set(to + size*2 + 1);
            }
        }
    }

    pub fn undo_move_false(&mut self, from: &AlgoMove, to: u8){
        match from {
            AlgoMove::NewP(size) => {
                self.unset(to + *size*2);
            }
            AlgoMove::MoveP(index, size) => {
                self.set(index + size*2); // enable the from field
                self.unset(index + size*2 + 1);

                self.unset(to + size*2); // disable the to field
            }
        }
    }

    pub fn undo_move_true(&mut self, from: &AlgoMove, to: u8){
        match from {
            AlgoMove::NewP(size) => {
                self.unset(to + *size*2);
            }
            AlgoMove::MoveP(index, size) => {
                self.set(index + size*2); // enable the from field
                self.set(index + size*2 + 1);

                self.unset(to + size*2); // disable the to field
            }
        }
    }

    pub fn check_win(&self) -> AlgoCheckRes {
        const WIN_ROWS: [[u8; 3]; 8] =
        [
            [
                0,
                6,
                12
            ],
            [
                18,
                24,
                30
            ],
            [
                36,
                42,
                48
            ],
            [
                0,
                18,
                36
            ],
            [
                6,
                24,
                42
            ],
            [
                12,
                30,
                48
            ],
            [
                0,
                24,
                48
            ],
            [
                36,
                24,
                12
            ]
        ];

        let mut out = AlgoCheckRes::Continue;

        for row in &WIN_ROWS{
            if let Some(sign) = self.check_row(row){
                match out {
                    AlgoCheckRes::Win(s) => {
                        if s != sign{
                            return AlgoCheckRes::Draw;
                        }
                    }
                    _ => {
                        out = AlgoCheckRes::Win(sign)
                    }
                }
            }

        }
        out
    }
 
    fn check_row(&self, row: &'static[u8; 3]) -> Option<bool>{
        let first = if self.get(row[0]+4){
            self.get(row[0]+5)
        }else if self.get(row[0]+2){
            self.get(row[0]+3)
        }else if self.get(row[0]){
            self.get(row[0]+1)
        }
        else {
            return None;
        };
        let second = if self.get(row[1]+4){
            self.get(row[1]+5)
        }else if self.get(row[1]+2){
            self.get(row[1]+3)
        }else if self.get(row[1]){
            self.get(row[1]+1)
        }
        else {
            return None;
        };
        if first == second{
            let third = if self.get(row[1]+4){
                self.get(row[1]+5)
            }else if self.get(row[1]+2){
                self.get(row[1]+3)
            }else if self.get(row[1]){
                self.get(row[1]+1)
            }
            else {
                return None;
            };
            if first == third{
                return Some(first);
            }
        }
        None
    }

    fn set(&mut self, i: u8){
        self.0 |= 1 << i
    }

    fn unset(&mut self, i: u8){
        self.0 &= !(1 << i);
    }

    fn toggle(&mut self, i: u8){
        self.0 ^= 1 << i
    }

    fn get(&self, i: u8) -> bool{
        if self.0 ^ (1 << i) != 0{
            true
        }
        else{
            false
        }
    }

    fn set_p(&mut self, i: u8){
        self.1 |= 1 << i
    }

    fn unset_p(&mut self, i: u8){
        self.1 &= !(1 << i);
    }

    fn toggle_p(&mut self, i: u8){
        self.1 ^= 1 << i
    }

    fn get_p(&self, i: u8) -> bool{
        if self.1 ^ (1 << i) != 0{
            true
        }
        else{
            false
        }
    }
}
