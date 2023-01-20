mod board;
mod p2_game;
mod cmp_game;

pub use p2_game::P2Scene;
pub use board::*;

pub enum Move{
	NewP(Piece),
	MoveP(usize, usize)
}
