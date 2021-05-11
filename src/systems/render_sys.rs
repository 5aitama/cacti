
use crate::components::{
    mesh_2d::Mesh2D,
};

use crate::core::{
    sys::Sys,
    component_tuple::ComponentTuple,
    world_state::WorldState,
};

pub struct RenderSys;
impl Sys for RenderSys {

    fn on_start(&self, _: &mut WorldState) { }

    fn on_update(&self, world_state: &mut WorldState) {
        for entity in <(Mesh2D,)>::get_entities(&world_state).iter() {
            let m = world_state.get_component_mut::<Mesh2D>(&entity).unwrap();
            m.draw();
            // println!("Vertex amount: {}", m.vertices.len());
        }
    }
}