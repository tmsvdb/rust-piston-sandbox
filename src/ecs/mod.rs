

mod systems;
mod components;

use std::time::*;
use ecs::systems::*;
use ecs::components::*;

/*
    ECS entity
*/
pub struct ECSEntity {
}

impl ECSEntity {
    fn add_component (&mut self) {

    }
}

/*
    ECS Core
*/
pub struct ECSCore {
    entities: Vec<ECSEntity>,
    time: Instant,
}
/*
impl ECSCore {
    pub fn new () -> ECSCore {
        ECSCore {
            entities: Vec::new(),
            time: Instant::now(),
        }
    }

    pub fn update (&mut self) {
        let delta_time = self.time.elapsed();
        self.time = Instant::now();

        for system in &self.systems {
            for entity in &self.entities {
                system.update(entity);
            }
        }
    }

    fn create_entity (&mut self) -> ECSEntity {
        let entity = ECSEntity { components: Vec::new() };
        self.entities.push(entity);
        return entity;
    }

}
*/
