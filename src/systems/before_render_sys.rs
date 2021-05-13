
extern crate gl;
extern crate glfw;

use crate::core::{sys::Sys, world::{EntityComponentManager, EntitySelector}};


use crate::components::{
    window::Window,
};

pub struct BeforeRenderSys;

impl Sys for BeforeRenderSys {

    fn on_update(&self, world: &mut EntityComponentManager) {
        match <(Window,)>::query_first_from(&world) {
            Some(entity) => {
                let window_component = world.get_component::<Window>(&entity).unwrap();

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