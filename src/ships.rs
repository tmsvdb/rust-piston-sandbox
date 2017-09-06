use piston_window::*;
use draw::*;

pub enum ShipTypes {
    Drone,
    Scout,
    Corvette,
    Destroyer,
    Frigrate,
    Cruiser,
    Battlecruiser,
    Carrier,
    Dreadnought
}

pub trait Ship {
    fn ship_type() -> ShipTypes;
    fn discription() -> String;
}

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
        polygon([1.0,1.0,1.0,1.0], &[[0.0,-30.0],[-25.0,15.0],[25.0,15.0]], c.transform.trans(100.0,100.0).rot_rad(self.rotation), g);
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

/*
Drones: Tiny unmanned patrol and spy vessels, around 2-3 meters in length. Their small mass and economical ion-propultion engines, make them hard to track and cheap to employ.

Scouts: Small vessels, typically about 50ish meters in length. Typically good for chasing down pirates and policing. Typically outfitted with point defenses or high yield torpedoes on real military operations.

Corvettes: Roughly 100-250 meters in length. Serves as light support/patrol.

Destroyer: About 400-500 meters in length. Now we're getting to the real fighters. Destroyers typically are meant to hunt down
corvettes, patrol boats and smaller vessels where the bigger ships can't be bothered.

Frigates: Clocking at around 600 meters, frigates are fairly well armed and armored. Capable of carrying out solo operations as much as supporting larger fleet actions, Frigates are versatile vessels who see action on almost any military situation.

Cruiser: 800 meter long warships that are hard-hitting frontliners. Fairly quick though somewhat lightly armored for their size, which serves as the distinction from their cousins, the Battlecruisers.

Battlecruiser: 800 meter long warships which can take punishment as much as they can give. They're not all that quick, but they're armed and armored quite well. Typically used as flagships for small fleet operations.

Battleship: 1000 meter long vessels that are a step above the battlecruiser. Also has small support vessel hangers for additional fleet actions.

Carriers: Roughly the same size as the battleship. Serves to carry large fighter squadrons into combat.

Dreadnoughts: The big boys. Over a mile long and having guns that can typically one-shot other ships, these things are fearsome to behold.

--

Starcruisers are large vessels, built to act as an FTL communication hub/troop carrier. Their capacity to receive and process data makes them a prime choice for a Flagship, however they aren't heavily armed, and tend to hang back during fleet engagements, depending on their escorts to keep them safe should the enemy move to engage them. During a planetary invasion, the command personnel aboard the Hubship coordinate the fleet to open up a landing corridor. Once the corridor has been established, the Starcruiser takes position in orbit above the planet and begins deploying dropships full of several divisions' worth of soldiers. The officers on the ground then coordinate with the command personnel aboard the Hubship. A single Hubship can coordinate an entire theatre of ground operations, while simultaneously coordinating fleet engagements.

Destroyers are as they always have been: heavily armed and armored warships that dominate the main lines of a battle. They tend to be armed with large, long-range, axial mounted lasers that can put accurate, devastating fire on-target at incredible ranges.
Destroyers are medium-sized vessels designed to engage the enemy at closer ranges than Battleships. Instead of having a large axial gun mounted along the spine of the vessel like a Battleship, Destroyers are built more stout and compact, and are outfitted with medium-range turrets, and close-range missile bays.

Corvettes are small, fast, and nimble reconnaissance ships that dance around enemy formations and rely on their onboard Electronic Countermeasures to baffle the enemy's targeting systems. A corvette uses its dedicated sensor suite and communications array to relay more accurate targeting information to the rest of the fleet.

Carrier are the workhorse of the fleet. They're highly modular and can fit many roles, from troop transport to heavy patrol craft to research vessel. They tend to be larger and less armored than Destroyers.

Frigates are light patrol craft, larger than Corvettes, yet smaller than destroyers. They patrol the edges of critical systems, monitoring ships as the drop from FTL travel, watching for any potential threats, or wanted ne'er-do-wells.
*/
