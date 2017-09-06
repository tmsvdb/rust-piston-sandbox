use piston_window::*;



/*
    Generig graphic type for object rendering
*/

pub trait Graphic {
    fn draw (&self, c : Context, g : &mut G2d);
}



/*
    Example code for implementaion of trait Graphic
*/

pub type Color = [f32; 4];
pub type Vector2 = (f64, f64);

pub struct DrawRect {
    pub color: Color,
    pub size: Vector2,
    pub position: Vector2,
    pub rotation: f64,
}

pub struct DrawEllipse {
    pub color: Color,
    pub size: Vector2,
    pub position: Vector2,
    pub rotation: f64,
}

pub struct DrawPolygon {
    pub color: Color,
    pub vectors: [[f64; 2]; 3],
    pub position: Vector2,
    pub rotation: f64,
}

impl Graphic for DrawRect {
    fn draw (&self, c : Context, g : &mut G2d) {
        rectangle(self.color, [0.0, 0.0, self.size.0, self.size.1], c.transform.trans(self.position.0, self.position.1), g);
    }
}

impl Graphic for DrawEllipse {
    fn draw (&self, c : Context, g : &mut G2d) {
        ellipse(self.color, [0.0, 0.0, self.size.0, self.size.1], c.transform.trans(self.position.0, self.position.1), g);
    }
}

impl Graphic for DrawPolygon {
    fn draw (&self, c : Context, g : &mut G2d) {
        polygon(self.color, &[self.vectors[0], self.vectors[1], self.vectors[2]], c.transform.trans(self.position.0, self.position.1), g);
    }
}

/*

pub struct Graphic {
    pub color: Color,
    pub size: Vector2,
    pub position: Vector2,
    pub rotation: f64,
}

impl Graphic {
    pub fn new (new_color: Color, new_size: Vector2, new_pos: Vector2, new_rot: f64) -> Graphic {
        Graphic {
            color: new_color,
            size: new_size,
            position: new_pos,
            rotation: new_rot
        }
    }
}


pub trait DrawEllipse {
    fn draw (&self, c : Context, g : &mut G2d);
}

pub trait DrawRectangle {
    fn new () -> Graphic;
    fn draw (&self, c : Context, g : &mut G2d);
}

pub trait DrawPolygon {
    fn draw (&self, c : Context, g : &mut G2d);
}

pub trait DrawText {
    fn draw (&self, c : Context, g : &mut G2d);
}

pub trait DrawImage {
    fn draw (&self, c : Context, g : &mut G2d);
}

impl DrawRectangle for Graphic {

    fn new () -> Graphic {
        Graphic::new([1.0,1.0,1.0,1.0], [100.0, 100.0], [50.0, 50.0], 35.0)
    }

    fn draw (&self, c : Context, g : &mut G2d) {

        let transform = c.transform
            //.rot_rad(self.rotation)
            .trans(self.position[0], self.position[1]);
            //.trans((self.rect[2]-self.rect[0])/2.0, (self.rect[3]-self.rect[1])/2.0);

        rectangle(self.color, [0.0, 0.0, self.size[0], self.size[1]], transform, g)
    }
}
*/
