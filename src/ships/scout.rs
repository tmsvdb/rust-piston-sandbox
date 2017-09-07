use piston_window::*;
use ships::*;

pub struct Scout {
    pub rotation: f64,
}

impl Scout {
    pub fn new () -> Scout {
        Scout { rotation: 0.0 }
    }
    pub fn rotate (&mut self, degree: f64) {
        self.rotation += degree;
    }
}

impl Graphic for Scout{
    fn draw (&self, c : Context, g : &mut G2d) {
        polygon(
            [1.0,1.0,1.0,1.0],
            &[[0.0,-30.0],[-25.0,15.0],[25.0,15.0]],
            c.transform.trans(100.0,100.0).rot_rad(self.rotation),
            g
        );
    }
}

impl Ship for Scout{
    fn ship_type () -> ShipTypes {
        ShipTypes::Scout
    }
    fn discription () -> String {
        String::from ("Scouts: Small vessels, typically about 50ish meters in length. Typically good for chasing down pirates and policing. Typically outfitted with point defenses or high yield torpedoes on real military operations.")
    }
}
