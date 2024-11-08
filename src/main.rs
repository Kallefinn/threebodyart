use nannou::prelude::*;
use nannou::glam::Vec2;
use nannou::rand::random_range;
use std::ops::Mul;
use nannou_egui::{self,egui,Egui};

const G: f32 = 0.5;

#[derive(PartialEq)]
struct Planet {

pos: Vec2,
mass: f32,
vel: Vec2,
colour: Srgba

}

struct Settings {
amount : u16,
color1 : Srgba, 
color2 : Srgba, 
color3 : Srgba,
trails : f32,
mass1 : f32,
mass2 : f32,
mass3 : f32,
offset : f32,

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
	nannou::app(model).update(update).run();
}

fn raw_window_event(_app : &App, model : &mut Model, event : &nannou::winit::event::WindowEvent) {
	model.egui.handle_raw_event(event);
} 
fn edit_hsv(ui: &mut egui::Ui, oldcolor: &mut Srgba) {
	let color : Hsv = (*oldcolor).into();
	let mut egui_hsv = egui::ecolor::Hsva::new(
        color.hue.to_positive_radians() as f32 / (std::f32::consts::PI * 2.0),
	color.saturation,
	color.value,
	1.0,
);
 
if egui::color_picker::color_edit_button_hsva(
	ui,
	&mut egui_hsv,
        egui::color_picker::Alpha::Opaque,
     )
     .changed()
	{
         *oldcolor = nannou::color::hsv(egui_hsv.h, egui_hsv.s, egui_hsv.v).into();
	}
}

struct Model {
systems : Vec<Vec<Planet>>,
settings : Settings,
egui : Egui,
}

fn new_system(settings : &Settings) -> Vec<Vec<Planet>> {
	
	let num1 = random_range(-300.0,300.0);
	let num2 = random_range(-300.0,300.0);
	let num3 = random_range(-300.0,300.0);
	let num4 = random_range(-300.0,300.0);
	let num5 = random_range(-300.0,300.0);
	let num6 = random_range(-300.0,300.0);

	let mut systems = Vec::with_capacity(settings.amount.into());

	let mut i = 0;
	while i < settings.amount {
	let offset = match settings.offset { 
		0.0 => 0.0,
		_ => random_range(-settings.offset,settings.offset),
	};
	systems.push(vec!{
		Planet{pos: Vec2::new(num1 + offset, num2 + offset),mass: settings.mass1,vel: Vec2::new(0.0,0.0), colour: settings.color1},
		Planet{pos: Vec2::new(num3 + offset, num4 + offset),mass: settings.mass2,vel: Vec2::new(0.0,0.0), colour: settings.color2},
		Planet{pos: Vec2::new(num5 + offset, num6 + offset),mass: settings.mass3,vel: Vec2::new(0.0,0.0), colour: settings.color3},

		});
	i += 1;
	}
	return systems
}

fn model(app: &App) -> Model {

let window_id = app
	.new_window()
	.title("three body art")
	.view(view)
	.raw_event(raw_window_event)
	.build()
	.unwrap();
let window = app.window(window_id).unwrap();
let egui = Egui::from_window(&window);

let settings = Settings {
	amount : 300,
	color1 : rgba(255.0,100.0,0.0,0.2),
	color2 : rgba(50.0,0.0,200.0,0.2),
	color3 : rgba(0.0,128.0,0.0,0.2),
	trails : 0.6,
	mass1 : 110.0,
	mass2 : 240.0,
	mass3 : 100.0,
	offset : 0.05	
};
	

let systems = new_system(&settings);

let model: Model = Model{systems : systems, settings : settings, egui : egui};

return model

}

fn calculations(planets: &mut Vec<Planet>) {
	
	let mut i = 0;
	while i < planets.len()-1 {
		let mut j = i+1;
		while j < planets.len() {
			let impulse = planets[i].force(&planets[j]);
			planets[i].vel += impulse;	
			planets[j].vel += impulse.mul(-1.0);
			j += 1;
		}

		i += 1;
	}
	for e in planets.iter_mut() {
		e.pos += e.vel;
	}
}

fn update(_app: &App, model: &mut Model, update: Update) {
 

let systems = &mut model.systems;
let egui = &mut model.egui;
let settings = &mut model.settings;

egui.set_elapsed_time(update.since_start);
let ctx = egui.begin_frame();

egui::Window::new("Settings").show(&ctx, |ui| {
	ui.label("Runtime: ");
	ui.label("");
	ui.label("Color Planet 1");
	edit_hsv(ui,&mut settings.color1);
	ui.label("Color Planet 2");
	edit_hsv(ui,&mut settings.color2);
	ui.label("Color Planet 3");
	edit_hsv(ui,&mut settings.color3);
	ui.label("Trails");
	ui.add(egui::Slider::new(&mut settings.trails, 0.0..=1.0));
	ui.label("Mass Planet 1");
	ui.add(egui::Slider::new(&mut settings.mass1, 1.0..=1000.0));
	ui.label("Mass Planet 2");
	ui.add(egui::Slider::new(&mut settings.mass3, 1.0..=1000.0));
	ui.label("Mass Planet 3");
	ui.add(egui::Slider::new(&mut settings.mass2, 1.0..=1000.0));
	ui.label("");
	ui.label("");
	ui.label("Regenerate to apply changes:");
	ui.label("");
	ui.label("Amount of systems");
	ui.add(egui::Slider::new(&mut settings.amount, 1..=100));
	ui.label("initial offset");
	ui.add(egui::Slider::new(&mut settings.offset, 0.0..=1.0));
	let clicked = ui.button("Regenerate").clicked();
	if clicked {
		*systems = new_system(&settings);
	}
});
for planets in systems.iter_mut() {
	planets[0].mass = settings.mass1;
	planets[0].colour = settings.color1;
	planets[1].mass = settings.mass2;
	planets[1].colour = settings.color2;
	planets[2].mass = settings.mass3;
	planets[2].colour = settings.color3;
	calculations(planets);

	}
}

fn draw_system(draw: &Draw, planets: &Vec<Planet>) {
	
	for planet in planets.iter() {
	draw.ellipse()
		.xy(planet.pos)
		.radius(planet.mass/10.0)
		.color(planet.colour);	
	}
}

fn view(app: &App, model: &Model, frame: Frame) {
    // Begin drawing
    let draw = app.draw();

    // Clear the background to blue.
   //	draw.background().color(rgba(0.0,0.0,0.0,0.001));
	let screen = app.window_rect();
	let settings = &model.settings;
	let trails = (settings.trails - 1.0) * -1.0;
	draw.rect().w(screen.w()).h(screen.h()).color(rgba(0.0,0.0,0.0,trails));
	

	for planets in model.systems.iter(){

		draw_system(&draw,&planets);
	}
    // Write the result of our drawing to the window's frame.
	draw.to_frame(app, &frame).unwrap();
	model.egui.draw_to_frame(&frame).unwrap();
}
