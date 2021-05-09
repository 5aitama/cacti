
extern crate gl;
extern crate glfw;

use crate::components::smc::SystemManagerComponent;
use self::glfw::{Action, Context, Key};

use crate::core::{
    sys::Sys,
    world_state::WorldState,
    component_tuple::ComponentTuple,
};

use crate::components::{
    window::Window,
};

pub struct WindowSys;

impl WindowSys {

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

impl Sys for WindowSys {
    
    fn on_start(&self, world_state: &mut WorldState) {
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

        glfw.window_hint(glfw::WindowHint::ContextVersion(4, 1));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
        
        #[cfg(target_os = "macos")]
        glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

        let size = (800u32, 600u32);
        let title = "My window";

        let (mut window, events) = glfw
            .create_window(size.0, size.1, title, glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW Window");

        window.make_current();
        window.set_key_polling(true);
        window.set_framebuffer_size_polling(true);

        // Load OpenGL methods...
        gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

        let window_component = Window {
            size: size,
            title: String::from(title),
            event: events,
            raw: window,
            glfw: glfw,
        };

        let e = world_state.create_entity().unwrap();
        world_state.add_component(&e, window_component);
    }

    fn on_update(&self, world_state: &mut WorldState) {
        match <(Window,)>::get_single_entity(&world_state) {
            Some(entity) => {
                let mut window_component = world_state.get_component_mut::<Window>(&entity).unwrap();

                if !window_component.raw.should_close() {
                    self.process_events(&mut window_component);

                    unsafe {
                        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
                        gl::ClearColor(0.1, 0.1, 0.1, 1.);
        
                        window_component.raw.swap_buffers();
                        window_component.glfw.poll_events();
                    }
                } else {
                    match <(SystemManagerComponent,)>::get_single_entity(&world_state) {
                        Some(e) => {
                            let mut smc = world_state.get_component_mut::<SystemManagerComponent>(&e).unwrap();
                            smc.shutdown = true;
                        },
                        None => {},
                    }
                }
            },
            None => {},
        }
    }
}