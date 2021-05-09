
use crate::core::{
    sys::Sys,
    world_state::WorldState,
    component_tuple::ComponentTuple,
};

use crate::components::{
    color::Color,
    position2d::Position2D,
};

pub struct PrintCubeSys;

impl Sys for PrintCubeSys {
    fn on_start(&self, _: &mut WorldState) { }

    fn on_update(&self, world_state: &mut WorldState) {

        // Print all entities that have the `Position2D` & `Color` components !
        for entity in <(Position2D, Color)>::get_entities(&world_state).iter() {
            let position2_d = world_state.get_component_ref::<Position2D>(&entity).unwrap();
            let color = world_state.get_component_ref::<Color>(&entity).unwrap();

            println!("Cube position: x:{} y:{}", position2_d.0, position2_d.1);
            println!("color: (r: {}, g: {}, b: {}, a: {})", color.0, color.1, color.2, color.3);
        }

    }

}