use raylib::prelude::*;

use super::board::*;
use crate::{Scene, utils::Button};

enum State{
	InGame,
	Win(&'static str)
}

pub struct P2Scene{
	board: Board,
	r_button: Button,
	swap_button: Button,
	new_game: Button,
	state: State
}

impl P2Scene {
	pub fn new(handle: &mut RaylibHandle, thread: &RaylibThread) -> Self{
		Self {
			board: Board::new(handle, thread, false),
			r_button: Button::default(handle, thread, "Assets/Buttons/return.png", 1090.0, 60.0, 170.0, 80.0),
			swap_button: Button::default(handle, thread, "Assets/Buttons/swap.png", 600.0, 60.0, 100.0, 50.0),
			new_game: Button::default(handle, thread, "Assets/Buttons/restart.png", 600.0, 60.0, 150.0, 50.0),
			state: State::InGame
		}
	}

    pub fn run(&mut self, handle: &mut RaylibHandle, thread: &RaylibThread) -> Scene{
    	while !handle.window_should_close(){
    		match self.state {
        		State::InGame => {
        			if let Some(s) = self.update(handle){
    					return s;
    				}
    				self.render(handle.begin_drawing(thread));
        		}
        		State::Win(name) => {

        			// update
        			{
        				let m_pos = handle.get_mouse_position();
    					self.r_button.update(&m_pos);
	    				self.new_game.update(&m_pos);
	    				if handle.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON){
				    		if self.new_game.is_clicked{
					    		self.board.reset();
					    		self.state = State::InGame;
					    	}
					    	if self.r_button.is_clicked{
					    		return Scene::Menu
					    	}
				    	}
        			}

			    	// render
			    	{
			    		let mut d = handle.begin_drawing(thread);
			    		d.clear_background(Color::SKYBLUE);
			    		self.board.render(&mut d);
			    		self.new_game.render(&mut d);
			    		self.r_button.render(&mut d);
			    		d.draw_text(name, 180, 300, 120, Color::GOLD)
			    	}
        		}
    		}
    	}
    	Scene::Exit
    }

    fn render(&mut self, mut d: RaylibDrawHandle){
    	d.clear_background(Color::SKYBLUE);

    	self.board.render(&mut d);
    	self.swap_button.render(&mut d);
    	self.r_button.render(&mut d);
    }

    fn update(&mut self, handle: &mut RaylibHandle) -> Option<Scene>{
    	let m_pos = handle.get_mouse_position();
    	self.r_button.update(&m_pos);
    	self.swap_button.update(&m_pos);
    	if handle.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON){
    		if self.swap_button.is_clicked{
    			self.board.swap()
    		}
    		else if let Some((res, _)) = self.board.update_clicked(&m_pos){
    			match res {
       				CheckResult::Win(b) => match b{
            			true => self.state = State::Win("Orange has won!"),
            			false =>  self.state = State::Win("Green has won!"),
        			}
        			CheckResult::Draw => self.state = State::Win("Draw!"),
        			CheckResult::Continue => (),
    			}
    		}
    		if self.r_button.is_clicked{
    			return Some(Scene::Menu)
    		}
    		
    	}
    	None
    }
}
