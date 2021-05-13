
use crate::{components::{
    mesh_2d::Mesh2D,
}, core::world::EntityComponentManager};

use crate::core::{
    sys::Sys, 
    world::{
        EntitySelector,
    }
};

pub struct RenderSys;
impl Sys for RenderSys {

    fn on_update(&self, world_state: &mut EntityComponentManager) {
        for entity in <(Mesh2D,)>::query_from(&world_state) {
            let m = world_state.get_component_mut::<Mesh2D>(&entity).unwrap();
            m.draw();
        }
    }
}