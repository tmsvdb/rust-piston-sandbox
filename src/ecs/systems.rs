
use ecs::*;

/*
    ECS System
*/

pub trait ECSSystem {
    fn update(&self, entity: &ECSEntity);
}

enum ECSSystems {
    // add systems here --->
}
