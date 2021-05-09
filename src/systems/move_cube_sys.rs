
use crate::core::{
    sys::Sys,
    world_state::WorldState,
    component_tuple::ComponentTuple,
};

use crate::components::{ 
    tags::cube::TagCube, 
    position2d::Position2D,
};

pub struct MoveCubeSys;

impl Sys for MoveCubeSys {
    fn on_start(&self, _: &mut WorldState) { }
    
    fn on_update(&self, world_state: &mut WorldState) {

        // Iterate over all entities that have `TagCube` and `Position2D` components
        for entity in <(TagCube, Position2D)>::get_entities(&world_state).iter() {
            // Get mutable reference of `Position2D` component from the entity...
            let mut position2_d = world_state.get_component_mut::<Position2D>(&entity).unwrap();
            // Modify the `Position2D` component :)
            position2_d.0 += 6.0;
        }

    }

}