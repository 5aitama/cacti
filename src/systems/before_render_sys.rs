
extern crate gl;
extern crate glfw;

use crate::core::{
    sys::Sys,
    world_state::WorldState,
    component_tuple::ComponentTuple,
};

use crate::components::{
    window::Window,
};

pub struct BeforeRenderSys;

impl Sys for BeforeRenderSys {
    
    fn on_start(&self, _: &mut WorldState) { }

    fn on_update(&self, world_state: &mut WorldState) {
        match <(Window,)>::get_single_entity(&world_state) {
            Some(entity) => {
                let window_component = world_state.get_component_ref::<Window>(&entity).unwrap();

                if !window_component.raw.should_close() {
                    unsafe {
                        gl::FrontFace(gl::CW);
                        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
                        gl::ClearColor(0.15, 0.15, 0.15, 1.0);
                    }
                }
            },
            None => {},
        }
    }
}