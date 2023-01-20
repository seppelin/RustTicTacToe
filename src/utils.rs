use raylib::prelude::*;

pub struct Button{
	pub rectangles: [Rectangle; 2],
	pub textures: [Texture2D; 2],
	pub is_clicked: bool
}

impl Button {
    pub fn default(handle: &mut RaylibHandle, thread: &RaylibThread, path: &'static str, x: f32, y: f32, w: f32, h: f32) -> Self {
    	
    	let rectangle = Rectangle::new(x - w/2.0, y - h/2.0, w, h);
    	
    	let mut img = Image::load_image(path).unwrap();
    	img.resize(rectangle.width as i32, rectangle.height as i32);

    	let mut p_img = img.clone();
    	p_img.resize((rectangle.width * 1.02) as i32, (rectangle.height * 1.02) as i32);

    	let pressed_rect = Rectangle::new(x - p_img.width as f32/2.0, y - p_img.height as f32/2.0, p_img.width as f32, p_img.height as f32);

    	Button {
    		rectangles: [rectangle, pressed_rect],
    		textures: [handle.load_texture_from_image(thread, &img).unwrap(), handle.load_texture_from_image(thread, &p_img).unwrap()],
    		is_clicked: false
    	}
    }

    pub fn update(&mut self, mouse_pos: &Vector2){
    	self.is_clicked = cc_point_rect(&mouse_pos, &self.rectangles[self.is_clicked as usize])
    }

    pub fn render(&mut self, d: &mut RaylibDrawHandle){
    	d.draw_texture(&self.textures[self.is_clicked as usize],
    		self.rectangles[self.is_clicked as usize].x as i32,
    		self.rectangles[self.is_clicked as usize].y as i32, Color::WHITE)
    }
}

pub fn cc_point_rect(p: &Vector2, rect: &Rectangle) -> bool{
	if p.x > rect.x && p.y > rect.y && p.x < rect.x+rect.width && p.y < rect.y+rect.height{
		return true
	}
	false
}
