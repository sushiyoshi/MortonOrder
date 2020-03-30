extern crate piston;
extern crate opengl_graphics;
extern crate graphics;

#[cfg(feature = "include_glutin")]
extern crate glutin_window;

use opengl_graphics::{OpenGL, GlGraphics, Filter, GlyphCache, TextureSettings};
use piston::window::{ WindowSettings };
use piston::input::*;
use piston::event_loop::*;
#[cfg(feature = "include_glutin")]
use glutin_window::GlutinWindow as AppWindow;

use std::f64;
use piston_window::*;
use graphics::character::CharacterCache;
use std::clone::Clone;

mod consts;
mod colider;
mod obj;

use colider::{CCell};
use obj::{App};

//use colider::*;
fn main() {
	let mut cell = CCell::new();

	cell.push(Box::new(-1));
	cell.push(Box::new(2));
	cell.push(Box::new(1));
	cell.push(Box::new(2));
	cell.push(Box::new(-4));
	println!("ーーーlength:{}",*cell.length.borrow());
	println!("ーーーinitial state");
	let mut iter = cell.ProIter();
	while let Some(a) = iter.next() {
		println!("{}",a);
	}

	println!("ーーーlength:{}",*cell.length.borrow());
	println!("ーーーRemove negative numbers");
	let mut iter = cell.iter();
	while let Some(a) = iter.next() {
		println!("{}",a);
	}

	cell.push(Box::new(4));
	println!("ーーーlength:{}",*cell.length.borrow());
	println!("ーーーPush [4]");
	let mut iter = cell.iter();
	while let Some(a) = iter.next() {
		println!("{}",a);
	}

	let mut app = App::new();
    app.all();

	//println!(" {} {}",*cell.first.borrow().obj,*cell.last.borrow().obj);
}