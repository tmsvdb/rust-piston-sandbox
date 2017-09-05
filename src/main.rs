extern crate piston_window;
extern crate find_folder;

use piston_window::*;
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
    let mut glyphs = Glyphs::new(font, factory).unwrap();


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

    let graphic: Graphic = Graphic::new_rectangle();
    /*
        Execute gameloop
    */

    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g| {
            clear([0.5, 0.5, 1.0, 1.0], g);
            graphic.draw(c, g);
        });

        if e.press_args().is_some() {
            InnerApp {
                title: "Inner loop (press X to exit inner loop)",
                exit_button: Button::Keyboard(Key::X),
            }.run(&mut window);
            window.set_title(title.into());
        }
    }
}
/*
fn draw (c : Context, g : &mut G2d) {
    clear([0.5, 1.0, 0.5, 1.0], g);
    rectangle([1.0, 0.0, 0.0, 1.0], [50.0, 50.0, 100.0, 100.0], c.transform, g);
    text([1.0, 0.0, 0.0, 1.0], 10, title.into(), &mut glyphs, c.transform.trans(10.0, 100.0), g);
    image(&rust_logo, c.transform, g);
}*/

/*
    Decouple render data
*/
pub type Color = [f32; 4];
pub type Rect = [f64; 4];
pub type Vector2 = [f64; 2];

enum GraphicStyle { Ellipse, Rectangle, Polygon, Text, Image }

struct Graphic {
    pub style: GraphicStyle,
    pub color: Color,
    pub rect: Rect,
    pub position: Vector2,
    pub rotation: f64,
}

trait Style {
    fn new_ellipse () -> Graphic;
    fn new_rectangle () -> Graphic;
    fn new_polygon () -> Graphic;
    fn new_text () -> Graphic;
    fn new_image () -> Graphic;
}

trait Draw {
    fn draw (&self, c : Context, g : &mut G2d);
}

impl Graphic {
    fn new (new_style: GraphicStyle, new_color: Color, new_rect: Rect, new_pos: Vector2, new_rot: f64) -> Graphic {
        Graphic {
            style: new_style,
            color: new_color,
            rect: new_rect,
            position: new_pos,
            rotation: new_rot
        }
    }
}

impl Style for Graphic {
    fn new_ellipse () -> Graphic {
        Graphic::new(GraphicStyle::Ellipse, [1.0, 1.0, 1.0, 1.0], [1.0, 1.0, 1.0, 1.0], [1.0, 1.0], 1.0)
    }
    fn new_rectangle () -> Graphic {
        Graphic::new(GraphicStyle::Rectangle, [1.0, 1.0, 1.0, 1.0], [50.0, 50.0, 100.0, 100.0], [100.0, 100.0], 35.0)
    }
    fn new_polygon () -> Graphic {
        Graphic::new(GraphicStyle::Polygon, [1.0, 1.0, 1.0, 1.0], [1.0, 1.0, 1.0, 1.0], [1.0, 1.0], 1.0)
    }
    fn new_text () -> Graphic {
        Graphic::new(GraphicStyle::Text, [1.0, 1.0, 1.0, 1.0], [1.0, 1.0, 1.0, 1.0], [1.0, 1.0], 1.0)
    }
    fn new_image () -> Graphic {
        Graphic::new(GraphicStyle::Image, [1.0, 1.0, 1.0, 1.0], [1.0, 1.0, 1.0, 1.0], [1.0, 1.0], 1.0)
    }
}

impl Draw for Graphic {
    fn draw (&self, c : Context, g : &mut G2d) {

        let transform = c.transform.trans(self.position[0], self.position[1])
            .rot_rad(self.rotation);
            //.trans((self.rect[2]-self.rect[0])/2.0, (self.rect[3]-self.rect[1])/2.0);

        match self.style {
            GraphicStyle::Ellipse => rectangle(self.color, self.rect, transform, g),
            GraphicStyle::Rectangle => rectangle(self.color, self.rect, transform, g),
            GraphicStyle::Polygon => rectangle(self.color, self.rect, transform, g),
            GraphicStyle::Text => rectangle(self.color, self.rect, transform, g),
            GraphicStyle::Image => rectangle(self.color, self.rect, transform, g),
        }
    }
}

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
