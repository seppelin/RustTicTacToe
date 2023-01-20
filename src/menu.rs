use raylib::prelude::*;

use super::{Scene, utils::Button, game};

pub(super) struct MenuScene{}

impl MenuScene {
    pub(super) fn run(handle: &mut RaylibHandle, thread: &RaylibThread) -> Scene{
		let mut buttons = [
			Button::default(handle, thread, "Assets/Buttons/new_game_2p.png", 600.0, 220.0, 200.0, 80.0),
			Button::default(handle, thread, "Assets/Buttons/new_game_cmp.png", 600.0, 320.0, 200.0, 80.0),
			Button::default(handle, thread, "Assets/Buttons/new_game_online.png", 600.0, 420.0, 200.0, 80.0),
			Button::default(handle, thread, "Assets/Buttons/exit.png", 100.0, 60.0, 150.0, 80.0)
		];

		let footer = handle.load_texture(thread, "Assets/Background/footer.png").unwrap();

		let mut mouse_pos;
		while !handle.window_should_close() {
	    	// update
	    	mouse_pos = handle.get_mouse_position();

	    	for i in &mut buttons{
	    		i.update(&mouse_pos)
	    	}

	    	if handle.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON){
	    		if buttons[0].is_clicked{
	    			return Scene::Game2P(game::P2Scene::new(handle, thread))
	    		}
	    		if buttons[3].is_clicked{
	    			return Scene::Exit;
	    		}
	    	}

	    	// render
	    	{	
	    		let mut d = handle.begin_drawing(thread);
	    		d.clear_background(Color::SKYBLUE);
	    		for i in &mut buttons{
	    			i.render(&mut d)
	    		}

	    		d.draw_texture(&footer, 200, 600, Color::WHITE);
	    	}
		}
		Scene::Exit
    }
}
