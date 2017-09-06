extern crate piston_window;
extern crate find_folder;

use piston_window::*;

mod draw;
use draw::*;

mod ships;
use ships::*;

//use find_folder::*;

fn main() {
    /*
        create new piston window
    */
    let title = "Hello Piston! (press any key to enter inner loop)";

    let mut window: PistonWindow = WindowSettings::new(title, [640, 480])
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|e| { panic!("Failed to build PistonWindow: {}", e) });

    /*
        Load text from assets folder
    */

    let assets: std::path::PathBuf = find_folder::Search::ParentsThenKids(3, 3).for_folder("assets").expect("folder [assets] not found!");
    let ref font: std::path::PathBuf = assets.join("font.ttf");
    if !font.exists() { panic!("file [{}] not found!", font.to_str().unwrap()) }
    let factory = window.factory.clone();
    let glyphs = Glyphs::new(font, factory).unwrap();


    /*
        Load image from assets folder
    */

    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets").expect("folder [assets] not found!");
    let rust_logo = assets.join("rust.png");
    if !rust_logo.exists() { panic!("file [{}] not found!", font.to_str().unwrap()) }
    let rust_logo = Texture::from_path(
            &mut window.factory,
            &rust_logo,
            Flip::None,
            &TextureSettings::new()
        ).unwrap();

    /*
        Create custom graphic
    */

    let mut ship: Scout = Scout::new();

    /*
        Execute gameloop
    */

    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g| {
            clear([0.5, 0.5, 1.0, 1.0], g);
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
