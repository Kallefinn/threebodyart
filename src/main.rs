use nannou::prelude::*;
use nannou::glam::Vec2;
use nannou::rand::random_range;
use std::ops::Mul;


const G: f32 = 0.5;

#[derive(PartialEq)]
struct Planet {

pos: Vec2,
mass: f32,
vel: Vec2,
colour: Srgba

}



impl Planet {
fn force(&self, other: &Planet) -> Vec2 {

	let dir = (other.pos - self.pos).normalize();
	let pull = G * self.mass * other.mass / self.pos.distance_squared(other.pos);
	let impulse = dir * pull.clamp(0.0,0.2);
	impulse
	}	
}



fn main() {

  
	nannou::app(model).update(update).simple_window(view).run();
}


struct Objects {
systems: Vec<Vec<Planet>>,
start_drawing: bool,
}

fn model(_app: &App) -> Objects {
		
	
	let num1 = random_range(-300.0,300.0);
	let num2 = random_range(-300.0,300.0);
	let num3 = random_range(-300.0,300.0);
	let num4 = random_range(-300.0,300.0);
	let num5 = random_range(-300.0,300.0);
	let num6 = random_range(-300.0,300.0);

let mut i = 0;

const COUNT :usize = 30;
let mut systems = Vec::with_capacity(COUNT);

while i < COUNT {
let offset = random_range(-0.05,0.05);
systems.push(vec!{
	Planet{pos: Vec2::new(num1 + offset, num2 + offset),mass: 110.0,vel: Vec2::new(0.0,0.0), colour: rgba(255.0,100.0,0.0,0.2)},
	Planet{pos: Vec2::new(num3 + offset, num4 + offset),mass: 240.0,vel: Vec2::new(0.0,0.0), colour: rgba(50.0,0.0,200.0, 0.2)},
	Planet{pos: Vec2::new(num5 + offset, num6 + offset),mass: 100.0,vel: Vec2::new(0.0,0.0), colour: rgba(0.0,128.0,0.0, 0.2)},

//	Planet{pos: Vec2::new(0.0, 0.0),mass: 500.0,vel: Vec2::new(0.0,0.0), colour: rgba(128.0,128.0,2.0, 1.0)},
});
i += 1;
}


let object: Objects = Objects{systems: systems, start_drawing: false};
object

}

fn calculations(objects: &mut Vec<Planet>) {
	
	let mut i = 0;
	while i < objects.len()-1 {
		if objects[i].mass == 500.0 {i += 1; continue;}
		let mut j = i+1;
		while j < objects.len() {
			let impulse = objects[i].force(&objects[j]);
			objects[i].vel += impulse;	
			objects[j].vel += impulse.mul(-1.0);
			j += 1;
		}

		i += 1;
	}
	for e in objects.iter_mut() {
		e.pos += e.vel;
	}
}

fn update(app: &App, world: &mut Objects, _update: Update) {
 

let systems = &mut world.systems;
for objects in systems.iter_mut() {
	calculations(objects);

	}
}

fn draw_system(draw: &Draw, objects: &Vec<Planet>) {
	
	for i in objects.iter() {
	if i.mass == 500.0 {continue;}
	//if i.mass == 240.0 {continue;}
	//if i.mass == 100.0 {continue;}
	draw.ellipse()
		.xy(i.pos)
		.radius(i.mass/10.0)
		.color(i.colour);	
	}
}

fn view(app: &App, world: &Objects, frame: Frame) {
    // Begin drawing
    let draw = app.draw();

    // Clear the background to blue.
   //	draw.background().color(rgba(0.0,0.0,0.0,0.001));
	let screen = app.window_rect();
	
	draw.rect().w(screen.w()).h(screen.h()).color(rgba(0.0,0.0,0.0,0.4));


for objects in world.systems.iter(){

	draw_system(&draw,&objects);
}
    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();
}
