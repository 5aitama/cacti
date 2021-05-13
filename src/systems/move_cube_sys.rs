
use crate::core::{sys::Sys, world::{EntityComponentManager, EntitySelector}};

use crate::components::{ 
    tags::cube::TagCube, 
    position2d::Position2D,
};

pub struct MoveCubeSys;

impl Sys for MoveCubeSys {

    fn on_update(&self, ecm: &mut EntityComponentManager) {
        // Iterate over all entities that have `TagCube` and `Position2D` components
        for entity in <(TagCube, Position2D)>::query_from(&ecm) {
            // Get mutable reference of `Position2D` component from the entity...
            let mut position2_d = ecm.get_component_mut::<Position2D>(&entity).unwrap();
            // Modify the `Position2D` component :)
            position2_d.0 += 6.0;
        }

    }

}