use self::algo_board::*;

use super::{BoardPiece, Move};

mod algo_board;

static MAX_DEPTH: i32 = 4;

pub struct Algo;

impl Algo {   
	pub fn run(b: &[[Vec<BoardPiece>; 3]; 3], next_sign: bool) -> Option<Move>{
		let board = AlgoBoard::from_board(b);

		None
	}

	fn min(board: &mut AlgoBoard, depth: i32) -> i32{
		if depth == MAX_DEPTH { return 0; }
		0
	}

	fn max(board: &mut AlgoBoard, depth: i32) -> i32{
		if depth == MAX_DEPTH { return 0; }

		match board.check_win() {
    		AlgoCheckRes::Win(sign) => {
    			match sign {
        			true => i32::MAX - depth,
        			false => i32::MIN + depth,
    			};
    		}
    		AlgoCheckRes::Draw => return 0,
    		AlgoCheckRes::Continue => {
    			let mut best_score = i32::MIN;
    			for field in 0..9{

    			}
    		}
		}
		0
	}
}

pub enum AlgoMove{
	NewP(u8),
	MoveP(u8, u8)
}

pub enum AlgoCheckRes{
	Win(bool),
	Draw,
	Continue,
}