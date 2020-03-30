#[cfg(feature = "include_glutin")]
extern crate glutin_window;
extern crate rand;
use rand::Rng;

use opengl_graphics::{OpenGL, GlGraphics, Filter, GlyphCache, TextureSettings};
use piston::window::{ WindowSettings };
use piston::input::*;
use piston::event_loop::*;
#[cfg(feature = "include_glutin")]
use glutin_window::GlutinWindow as AppWindow;

use graphics::rectangle::square;

use std::f64;
use piston_window::*;
use graphics::character::CharacterCache;
use std::clone::Clone;

use super::consts::*;
use super::colider;

pub struct App {
	list:Vec<GameObject>,
	gl: GlGraphics,
	window: AppWindow,
}


impl App {
	pub fn new() -> App {
		let opengl = OpenGL::V3_2;
		let window = App::new_window(opengl);
		App {
    		gl:GlGraphics::new(opengl),
    		window: window,
    		list :Vec::with_capacity(100),
    	}
	}
	fn new_window(opengl:OpenGL) -> AppWindow{
    	WindowSettings::new("shooting",[WIDTH as u32, HEIGHT as u32])
    	.exit_on_esc(true).graphics_api(opengl).build().unwrap()
  	}
  	pub fn all(&mut self) {
  		let mut events = Events::new(EventSettings::new());
  		let mut rnd = rand::thread_rng();
  		while let Some(e) = events.next(&mut self.window) {
  			self.list = GameObject::update_all(&self.list);
  			if let Some(r) = e.render_args(){ 
                self.render(&r);
            }         
            if let Some(_) = e.update_args() {
                self.update();
            }
  			if rnd.gen_range(0, 50) == 0 {
  				let mut obj = GameObject::new();
  				obj.size = rnd.gen_range(10.0,100.0);
  				obj.x = rnd.gen_range(0.0 - obj.size,WIDTH + obj.size);
  				obj.y = rnd.gen_range(0.0 - obj.size,HEIGHT + obj.size);
  				let tmp :i32 = rnd.gen_range(0,3);
  				match tmp {
  					0 => {
  						obj.x = 0.0 - obj.size;
  					}
  					1 => {
  						obj.x = WIDTH + obj.size;

  					}
  					2 => {
  						obj.y = 0.0 - obj.size;

  					}
  					3 => {
  						obj.y = HEIGHT + obj.size;

  					}
            _ => panic!("unchi")
  				}

  				obj.speed = rnd.gen_range(0.0,0.2);
          //obj.accele = rnd.gen_range(0.0,0.00);
          obj.dir = rnd.gen_range(0.0,360.0);         
          obj.dir_accele = rnd.gen_range(0.0,0.001);
          obj.color = [rnd.gen_range(0.0,1.0),rnd.gen_range(0.0,1.0),rnd.gen_range(0.0,1.0),0.3];
  				self.list.push(obj);
  			}
  		}
  	}
	fn render(&mut self, args: &RenderArgs) {
		let square: graphics::types::Rectangle = square(0.0, 0.0, 1.0);

		let list = self.list.clone();
		self.gl.draw(args.viewport(), |c, gl|{
            clear([0.0,0.0,0.0,1.0],gl);
            
            App::draw_obj(&c, gl, &list, square);
        });
	}
	fn draw_obj(c: &Context, gl: &mut GlGraphics, list: &Vec<GameObject>, square: graphics::types::Rectangle) {
		for obj in list {
			let cl:[f32; 4]  = obj.color;
			let transform = c.transform
                .trans(obj.x, obj.y);
            circle_arc(cl, obj.size,0.0,f64::consts::PI*1.9999,square,transform, gl);
		}
	}
	fn update(&mut self) {
		self.list = GameObject::update_all(&self.list);
	}
}

#[derive(Clone)]
struct GameObject {
    x:f64,
    y:f64,
    size:f64,
    dir:f64,
    speed:f64,
    accele:f64,
    //向きの加速度
    dir_accele:f64,
    color:[f32; 4],
}
impl Default for GameObject {
  fn default() -> GameObject {
    GameObject{x:0.0,
    y:0.0,
    size:5.0,
    dir:0.0,
    speed:1.0,
    accele:0.0,
    //向きの加速度
    dir_accele:0.0,
    color:[1.0; 4],}
  }
}
fn remove_not_use(list:Vec<GameObject>) -> Vec<GameObject> {
	    list
	        .into_iter()
	        .filter(|obj| !obj.isDestroy())
	        .collect()
}
impl GameObject {
	pub fn new() -> GameObject {
		Default::default()
	}
	fn update(&self) -> GameObject{
	    let math = f64::consts::PI/180.0 * self.dir;
	    let mut  ret = self.clone();
	    ret.x += (math.cos() - 0.0)*ret.speed;
	    ret.y += (math.sin() - 0.0)*ret.speed;
	    ret.speed += ret.accele;
	    ret.dir += ret.dir_accele;
	    ret
	}
	fn isDestroy(&self) -> bool{
		self.x > (WIDTH + self.size) || self.x < (0.0  - self.size) || self.y > (HEIGHT  + self.size) || self.y < (0.0  - self.size)
	}

	pub fn update_all(list :&Vec<GameObject>) -> Vec<GameObject> {
		let ret = list
			.into_iter()
			.map(|obj| obj.update())
			.collect();
		remove_not_use(ret)
	}
}
