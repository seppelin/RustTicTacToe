pub struct AlgoBoard{
	board: [[AlgoPiece; 3]; 9],
	pieces: [usize]
}

pub enum AlgoPiece{
	None,
	Small(bool),
	Mid(bool),
	Big(bool)
}