use raylib::{texture::Texture2D, RaylibHandle, RaylibThread, prelude::{RaylibDrawHandle, RaylibDraw, Color, Vector2, Rectangle}};
use std::cmp::Ordering;

use crate::utils::cc_point_rect;

static WIN_ROWS: [[[usize; 2]; 3]; 8] = [
	[
		[0, 0],
		[0, 1],
		[0, 2]
	],
	[
		[1, 0],
		[1, 1],
		[1, 2]
	],
	[
		[2, 0],
		[2, 1],
		[2, 2]
	],
	[
		[0, 0],
		[1, 0],
		[2, 0]
	],
	[
		[0, 1],
		[1, 1],
		[2, 1]
	],
	[
		[0, 2],
		[1, 2],
		[2, 2]
	],
	[
		[0, 0],
		[1, 1],
		[2, 2]
	],
	[
		[2, 0],
		[1, 1],
		[0, 2]
	]
];

enum BoardState {
    SelectedNewP(Piece),
    SelectedP(usize, usize),
    None
}

pub struct Board{
	grid: [[Vec<BoardPiece>; 3]; 3],
	grid_rects: [[Rectangle; 3]; 3],

	pieces_available: [[usize; 3]; 2],
	pieces_a_rects: [[Rectangle; 3]; 2],

	option_field_t: Texture2D,
	option_piece_t: Texture2D,
	pieces_t: [[[Texture2D; 2]; 3]; 2],
	grid_t: Texture2D,

	next_sign: bool,
	swap: bool,
	count: usize,
	options: [[[bool; 2]; 3]; 3],
	state: BoardState
}

impl Board {
    pub fn new(handle: &mut RaylibHandle, thread: &RaylibThread, swap: bool) -> Board{
    	// Grid
    	let grid: [[Vec<BoardPiece>; 3]; 3] = std::array::from_fn(|_| std::array::from_fn(|_| Vec::new()));
    	let grid_rects: [[Rectangle; 3]; 3] = std::array::from_fn(|w|{
    		std::array::from_fn(|h|{
    			Rectangle { x: (300 + w*200) as f32, y: (100 + h*200) as f32, width: 200.0, height: 200.0 }
    		})
    	});

    	// Pieces Available
    	let pieces_available = [[2;3];2];
    	let pieces_a_rects = 
    	[
    		[
				Rectangle::new(50.0, 300.0, 120.0, 120.0),
				Rectangle::new(50.0, 420.0, 160.0, 160.0),
				Rectangle::new(50.0, 580.0, 200.0, 200.0),
			],
			[
				Rectangle::new(1150.0-120.0, 300.0, 120.0, 120.0),
				Rectangle::new(1150.0-160.0, 420.0, 160.0, 160.0),
				Rectangle::new(1150.0-200.0, 580.0, 200.0, 200.0),
			]

    	];

    	// Textures
    	let pieces_t = [
    		[
    			[
    				handle.load_texture(thread, "Assets/Pieces/mini_0.png").unwrap(),
    				handle.load_texture(thread, "Assets/Pieces/mini_0_s.png").unwrap(),
    			],
    			[
    				handle.load_texture(thread, "Assets/Pieces/mid_0.png").unwrap(),
    				handle.load_texture(thread, "Assets/Pieces/mid_0_s.png").unwrap(),
    			],
    			[
    				handle.load_texture(thread, "Assets/Pieces/bigb_0.png").unwrap(),
    				handle.load_texture(thread, "Assets/Pieces/bigb_0_s.png").unwrap(),
    			]
    			
    			
    		],
    		[	
    			[
    				handle.load_texture(thread, "Assets/Pieces/mini_1.png").unwrap(),
    				handle.load_texture(thread, "Assets/Pieces/mini_1_s.png").unwrap(),
    			],
    			[
    				handle.load_texture(thread, "Assets/Pieces/mid_1.png").unwrap(),
    				handle.load_texture(thread, "Assets/Pieces/mid_1_s.png").unwrap(),
    			],
    			[
    				handle.load_texture(thread, "Assets/Pieces/bigb_1.png").unwrap(),
    				handle.load_texture(thread, "Assets/Pieces/bigb_1_s.png").unwrap(),
    			]
    		]
    	];
    	let grid_t = handle.load_texture(thread, "Assets/Grid/grid.png").unwrap();
    	let option_field_t = handle.load_texture(thread, "Assets/Grid/option_field.png").unwrap();
    	let option_piece_t = handle.load_texture(thread, "Assets/Grid/option_piece.png").unwrap();

    	Board { 
    		grid,
    		grid_rects,

    		pieces_available,
    		pieces_a_rects,

    		grid_t,
    		pieces_t,
    		option_field_t,
    		option_piece_t,

    		next_sign: false,
    		swap,
    		count: 0,
    		options: [[[false; 2]; 3]; 3],
    		state: BoardState::None
    	}
    }

    pub fn reset(&mut self) {
    	self.grid  = std::array::from_fn(|_| std::array::from_fn(|_| Vec::new()));
    	self.next_sign = false;
    	self.count = 0;
    	self.options = [[[false; 2]; 3]; 3];
    	self.pieces_available = [[2;3];2];
    	self.state = BoardState::None
    }

    pub fn new_p(&mut self, w: usize, h: usize, piece: Piece) -> Option<CheckResult>{
    	self.state = BoardState::None;
    	if self.pieces_available[self.next_sign as usize][piece.as_usize()] != 0{
    		unsafe{
    			if self.grid[w][h].len() != 0 &&
    			self.grid[w][h].last().unwrap_unchecked().piece >= piece {
    				return None;
    			}
    			else {
    				return Some(self.unchecked_new_p(w, h, piece));
    			}
    		}
    	}
    	return None;
    }

    pub fn move_p(&mut self, w: usize, h: usize, n_w: usize, n_h: usize) -> Option<CheckResult>{
    	self.state = BoardState::None;
    	unsafe{
    		if self.grid[w][h].len() != 0 && self.grid[w][h].last().unwrap_unchecked().sign == self.next_sign {
    			if self.grid[n_w][n_h].len() != 0 &&
    			self.grid[w][h].last().unwrap_unchecked().piece <= self.grid[n_w][n_h].last().unwrap_unchecked().piece {
    				return None;
    			}
    			else {
    				return Some(self.unchecked_move_p(w, h, n_w, n_h));
    			}
    		}
    	}
    	return None;
    }

    unsafe fn unchecked_move_p(&mut self, w: usize, h: usize, n_w: usize, n_h: usize) -> CheckResult{
    	let piece = self.grid[w][h].swap_remove(self.grid[w][h].len() - 1);
    	self.grid[n_w][n_h].push(piece);
    	self.count += 1;
    	self.next_sign = !self.next_sign;
    	check_win(&self.grid)
    }

    unsafe fn unchecked_new_p(&mut self, w: usize, h: usize, piece: Piece) ->CheckResult{
    	self.pieces_available[self.next_sign as usize][piece.as_usize()] -= 1;
    	self.grid[w][h].push(BoardPiece { piece, sign: self.next_sign });
    	self.count += 1;
    	self.next_sign = !self.next_sign;
    	check_win(&self.grid)
    }

    pub fn swap_next_sign(&mut self){
    	self.next_sign = !self.next_sign;
    	self.state = BoardState::None
    }

    pub fn get_next_sign(&self) -> bool{
    	return self.next_sign.clone();
	}

	pub fn get_count(&self) -> usize{
		return self.count.clone();
	}

	pub fn get_grid(&self) -> &[[Vec<BoardPiece>; 3]; 3]{
		&self.grid
	}

	pub fn render(&self, d: &mut RaylibDrawHandle){
		// render options
		for w in 0..3 {
			for h in 0..3 {
				if self.options[w][h][0]{
					if self.options[w][h][1]{
						d.draw_texture(&self.option_field_t,
							self.grid_rects[w][h].x as i32,
							self.grid_rects[w][h].y as i32, Color::WHITE)
					}
					else{
						d.draw_texture(&self.option_piece_t,
							self.grid_rects[w][h].x as i32,
							self.grid_rects[w][h].y as i32, Color::WHITE)
					}
				}
			}
		}

		// render grid
		d.draw_texture(&self.grid_t, 300, 100, Color::WHITE);

		// render pieces
		for i in 0..3{
			for n in 0..self.pieces_available[self.swap as usize][i]{
				d.draw_texture(&self.pieces_t[self.swap as usize][i][0], self.pieces_a_rects[0][i].x as i32 + n as i32*20, self.pieces_a_rects[0][i].y as i32, Color::WHITE)
			}
		}
		for i in 0..3{
			for n in 0..self.pieces_available[!self.swap as usize][i]{
				d.draw_texture(&self.pieces_t[!self.swap as usize][i][0], self.pieces_a_rects[1][i].x as i32 + n as i32*(-20), self.pieces_a_rects[1][i].y as i32, Color::WHITE)
			}
		}

		// render board pieces
		for w in 0..3{
			for h in 0..3{
				if self.grid[w][h].len() != 0{
					unsafe{
						let last = self.grid[w][h].last().unwrap_unchecked();
						let offset = match self.grid[w][h].last().unwrap_unchecked().piece.as_usize() {
							0 => 40,
							1 => 20,
							2 => 0,
						    _ => panic!("Maths broke as usize")
						};
						d.draw_texture(&self.pieces_t[last.sign as usize][last.piece.as_usize()][0],
							self.grid_rects[w][h].x as i32 + offset,
							self.grid_rects[w][h].y as i32 + offset, Color::WHITE);
					}
				}
			}
		}

		// render selected
		match self.state {
    		BoardState::SelectedNewP(piece) => {
    			let s = match self.next_sign^self.swap {
        			true => -1,
        			false => 1,
    			};
    			let i = piece.as_usize();
    			let n = self.pieces_available[self.next_sign as usize][i] - 1;
    			d.draw_texture(&self.pieces_t[self.next_sign as usize][i][1],
    				self.pieces_a_rects[(self.next_sign^self.swap) as usize][i].x as i32 + n as i32*20*s,
    				self.pieces_a_rects[(self.next_sign^self.swap) as usize][i].y as i32,
    				Color::WHITE)
    		}
    		BoardState::SelectedP(w, h) => unsafe {
    			let last = self.grid[w][h].last().unwrap_unchecked();
    			let offset = match self.grid[w][h].last().unwrap_unchecked().piece.as_usize() {
					0 => 40,
					1 => 20,
					2 => 0,
					 _ => panic!("Maths broke as usize")
				};
				d.draw_texture(&self.pieces_t[last.sign as usize][last.piece.as_usize()][1],
					self.grid_rects[w][h].x as i32 + offset,
					self.grid_rects[w][h].y as i32 + offset, Color::WHITE);
    		}
    		BoardState::None => (),
		}
	}

	pub fn swap(&mut self){
		self.swap = !self.swap;
	}

	fn update_options(&mut self){
		self.options = [[[false; 2]; 3]; 3];
		match self.state {
    		BoardState::SelectedNewP(p) => unsafe {
    			for w in 0..3{
    				for h in 0..3{
    					if self.grid[w][h].len() != 0 {
    						if self.grid[w][h].last().unwrap_unchecked().piece < p {
    							self.options[w][h][0] = true;
    						}
    					}
    					else {
    						self.options[w][h][0] = true;
    						self.options[w][h][1] = true;
    					}
    				}
    			}
    		}
    		BoardState::SelectedP(w_p, h_p) => unsafe {
    			for w in 0..3{
    				for h in 0..3{
    					if w_p == w && h_p == h{
    						continue;
    					}
    					if self.grid[w][h].len() != 0 {
    						if self.grid[w][h].last().unwrap_unchecked().piece < self.grid[w_p][h_p].last().unwrap_unchecked().piece {
    							self.options[w][h][0] = true;
    						}
    					}
    					else {
    						self.options[w][h][0] = true;
    						self.options[w][h][1] = true;
    					}
    				}
    			}
    		}
    		BoardState::None => (),
		}
	}

	pub fn update_clicked(&mut self, p: &Vector2) -> Option<(CheckResult, (usize, usize))>{
		for w in 0..3{
			for h in 0..3{
				if cc_point_rect(&p, &self.grid_rects[w][h]) { 
					unsafe{
						match self.state {
		        	BoardState::SelectedNewP(piece) => {
	        			return if self.options[w][h][0]{
	        				self.state = BoardState::None;
		       				self.unchecked_new_p(w, h, piece);
	        				self.update_options();
	        				Some((check_win(&self.grid), (w, h)))
	        			}
	        			else{
	        				if let Some(p) = self.grid[w][h].last(){
	        					if p.sign == self.next_sign{
	        						self.state = BoardState::SelectedP(w, h);
	        						self.update_options();
	        						return None
	        					}
	        				}
	        				self.state = BoardState::None;
	        				self.update_options();
	        				None
	        			}
		       		},

		       		BoardState::SelectedP(o_w, o_h) => {
		       			return if self.options[w][h][0]{
	        				self.state = BoardState::None;
		       				self.unchecked_move_p(o_w, o_h, w, h);
	        				self.update_options();
	        				Some((check_win(&self.grid), (w, h)))
	        			}
	        			else{
	        				if self.grid[w][h].last().unwrap_unchecked().sign == self.next_sign && w != o_w && h != o_h{
	        					self.state = BoardState::SelectedP(w, h);
	        					self.update_options();
	        					return None
	        				}
	        				self.state = BoardState::None;
	        				self.update_options();
	        				None
	        			}
		       		}
		       		BoardState::None => {
		       			if let Some(p) = self.grid[w][h].last(){
		       				if p.sign == self.next_sign{
		       					self.state = BoardState::SelectedP(w, h);
		       					self.update_options();
		       					return None;
		       				}
		       			}
		       			return None;
		       		}
		   			}
					}
				}
			}
		}
		for i in 0..3{
			let offset;
			if self.next_sign^self.swap{
				offset = self.pieces_available[self.next_sign as usize][i] as f32 * -20.0;
			}
			else {
				offset = self.pieces_available[self.next_sign as usize][i] as f32 * 20.0;
			}
			let rect = &self.pieces_a_rects[(self.next_sign^self.swap) as usize][i];
			if cc_point_rect(&p, &Rectangle { x: rect.x + offset, y: rect.y, width: rect.width, height: rect.height }){
				match &self.state{
        			BoardState::SelectedNewP(ref p) => {
        				if p.as_usize() == i{
        					self.state = BoardState::None;
		       				self.update_options();
        					return None;
        				}
        			}
        			_ => ()
    			}
				self.state = BoardState::SelectedNewP(match i {
					   0 => Piece::Mini,
					   1 => Piece::Mid,
					   2 => Piece::Big,
					   _ => panic!("Math broke")
				});
				self.update_options();
				return None;
			}
		}
		self.state = BoardState::None;
		self.update_options();
        return None;
	}
}

pub fn check_win(board: &[[Vec<BoardPiece>; 3]; 3]) -> CheckResult{
	let mut win = [false; 2];

	for row in &WIN_ROWS{
		check_row(board, row, &mut win)
	}

	return if win[0]{
		if win[1] {
			CheckResult::Draw
		}
		else {
			CheckResult::Win(false)
		}
	}
	else if win[1] {
		CheckResult::Win(true)
	}
	else {
		CheckResult::Continue
	}
}

fn check_row(board: &[[Vec<BoardPiece>; 3]; 3], row: &[[usize; 2]; 3], win: &mut [bool; 2]){
	if  board[row[0][0]][row[0][1]].len() != 0 && board[row[1][0]][row[1][1]].len() != 0 && board[row[2][0]][row[2][1]].len() != 0 {
		unsafe {
			if board[row[0][0]][row[0][1]].last().unwrap_unchecked().sign == board[row[1][0]][row[1][1]].last().unwrap_unchecked().sign &&
			board[row[0][0]][row[0][1]].last().unwrap_unchecked().sign == board[row[2][0]][row[2][1]].last().unwrap_unchecked().sign{
				win[board[row[0][0]][row[0][1]].last().unwrap_unchecked().sign as usize] = true;
			}
		}
	}
}

pub enum CheckResult{
	Win(bool),
	Draw,
	Continue
}

#[derive(Clone, Copy, PartialEq)]
pub enum Piece{
	Mini,
	Mid,
	Big
}

impl Piece {
    pub fn as_usize(&self) -> usize {
    	return match self {
        	Piece::Mini => 0,
        	Piece::Mid => 1,
        	Piece::Big => 2,
    	}
    }
}

impl PartialOrd for Piece {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        return match self {
            Piece::Mini => match other {
                Piece::Mini => Some(Ordering::Equal),
                Piece::Mid => Some(Ordering::Less),
                Piece::Big => Some(Ordering::Less),
            },
            Piece::Mid => match other {
                Piece::Mini => Some(Ordering::Greater),
                Piece::Mid => Some(Ordering::Equal),
                Piece::Big => Some(Ordering::Less),
            },
            Piece::Big => match other {
                Piece::Mini => Some(Ordering::Greater),
                Piece::Mid => Some(Ordering::Greater),
                Piece::Big => Some(Ordering::Equal),
            }
        }
    }
}

#[derive(Clone, Copy)]
pub struct BoardPiece {
	pub piece: Piece,
	pub sign: bool,
}
