use piston_window::*;
use draw::*;

pub mod scout;

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
    fn ship_type () -> ShipTypes;
    fn discription () -> String;
    //fn steering () -> Steering;
    //fn engine () -> Engine;
    //fn hull () -> Hull;
}

/*
    Ship components
*/

pub struct Hull {
    mass: f64,
    strength: f64,
    damage: f64,
}

pub struct Engine {
    speed: f64,
    thrust: f64,
}

pub struct Steering {
    rotation: f64,
    torque: f64,
}


/*
About spaceships:
-----------------

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
*/
