
extern crate gl;
extern crate glfw;

use self::glfw::{Action, Context, Key};

use crate::core::{sys::Sys, world::{EntityComponentManager, EntitySelector}};

use crate::components::{
    window::Window,
};

pub struct AfterRenderSys;

impl AfterRenderSys {

    fn process_events(&self, window_component: &mut Window) {
        for (_, event) in glfw::flush_messages(&window_component.event) {
            match event {
                glfw::WindowEvent::FramebufferSize(width, height) => unsafe {
                    gl::Viewport(0, 0, width, height);
                },
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    window_component.raw.set_should_close(true);
                }
                _ => {}
            }
        }
    }

}

impl Sys for AfterRenderSys {

    fn on_update(&self, world_state: &mut EntityComponentManager) {
        match <(Window,)>::query_first_from(&world_state) {
            Some(entity) => {
                let mut window_component = world_state.get_component_mut::<Window>(&entity).unwrap();

                if !window_component.raw.should_close() {
                    self.process_events(&mut window_component);
                    window_component.raw.swap_buffers();
                    window_component.glfw.poll_events();
                }
            },
            None => {},
        }
    }
}