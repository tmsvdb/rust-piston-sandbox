extern crate piston_window;
extern crate find_folder;

use piston_window::*;

mod ecs;
use ecs::*;

mod draw;
use draw::*;

mod ships;
use ships::*;
use ships::scout::*;

use std::time::{Duration, Instant};

mod assets;
use assets::*;



//use find_folder::*;

fn main() {

    /*
        Create new ECS engine
    */
    //let ecs = ECS::new();

    /*
        create new piston window
    */
    let title = "Hello Piston! (press any key to enter inner loop)";

    let mut window: PistonWindow = WindowSettings::new(title, [640, 480])
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|e| { panic!("Failed to build PistonWindow: {}", e) });

    let mut assets : Assets = Assets::new(&window);
    let glyphs = assets.load_font ("font.ttf");
    let rust_logo = assets.load_image ("rust.png");
    /*
        Create custom graphic
    */

    let mut ship: Scout = Scout::new();

    /*
        Execute gameloop
    */

    let mut time = Instant::now();
    let mut count = 0;
    let mut fps: String = "fps: 0".into();
    let sample_rate : u64 = 25;

    while let Some(e) = window.next() {

        if count > sample_rate {
            fps = format!("fps: {}", sample_rate as f64 * (1.0 / ((time.elapsed().subsec_nanos() as f64)/1_000_000_000.0)));
            time = Instant::now();
            count = 0;
        }
        else {
            count += 1;
        }
        window.draw_2d(&e, |c, g| {
            clear([0.5, 0.5, 1.0, 1.0], g);
            image(&rust_logo.texture, c.transform, g);
            text([1.0, 0.0, 0.0, 1.0], 30, &fps, &mut glyphs.glyphs, c.transform.trans(10.0, 100.0), g);
            ship.draw(c, g);
        });

        if e.press_args().is_some() {
            InnerApp {
                title: "Inner loop (press X to exit inner loop)",
                exit_button: Button::Keyboard(Key::X),
            }.run(&mut window);
            window.set_title(title.into());
        }

        ship.rotate(0.01);
    }
}
/*
fn draw (c : Context, g : &mut G2d) {
    clear([0.5, 1.0, 0.5, 1.0], g);
    rectangle([1.0, 0.0, 0.0, 1.0], [50.0, 50.0, 100.0, 100.0], c.transform, g);
    text([1.0, 0.0, 0.0, 1.0], 10, title.into(), &mut glyphs, c.transform.trans(10.0, 100.0), g);
    image(&rust_logo, c.transform, g);
}*/



/// Stores application state of inner event loop.
pub struct InnerApp {
    pub title: &'static str,
    pub exit_button: Button,
}

impl InnerApp {
    pub fn run(&mut self, window: &mut PistonWindow) {
        window.set_title(self.title.into());
        while let Some(e) = window.next() {
            window.draw_2d(&e, |c, g| {
                clear([0.5, 0.5, 1.0, 1.0], g);
                //ellipse([1.0, 0.0, 0.0, 1.0], [50.0, 50.0, 100.0, 100.0], c.transform, g);
                let poly = &[[50.0, 50.0],[100.0, 50.0], [100.0, 100.0], [50.0, 100.0]];
                polygon([0.0, 1.0, 0.0, 1.0], poly, c.transform, g);
            });
            if let Some(button) = e.press_args() {
                if button == self.exit_button {
                    break;
                }
            }
        }
    }
}
