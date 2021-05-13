
use crate::core::{sys::Sys, world::{EntityComponentManager, EntitySelector}};

use crate::components::{
    color::Color,
    position2d::Position2D,
};

pub struct PrintCubeSys;

impl Sys for PrintCubeSys {
    
    fn on_update(&self, world: &mut EntityComponentManager) {

        // Print all entities that have the `Position2D` & `Color` components !
        for entity in <(Position2D, Color)>::query_from(&world) {
            let position2_d = world.get_component::<Position2D>(&entity).unwrap();
            let color = world.get_component::<Color>(&entity).unwrap();

            println!("Cube position: x:{} y:{}", position2_d.0, position2_d.1);
            println!("color: (r: {}, g: {}, b: {}, a: {})", color.0, color.1, color.2, color.3);
        }

    }

}
