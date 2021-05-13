
extern crate gl;
extern crate glfw;

use crate::{components::smc::SystemManagerComponent, core::world::EntityComponentManager};
use self::glfw::{Action, Context, Key};

use crate::core::{
    sys::Sys, 
    world::{
        EntitySelector,
    }
};
use crate::components::{
    window::Window,
};

pub struct WindowSys {
    title: &'static str,
    size: (u32, u32),
}

impl WindowSys {

    pub fn new(title: &'static str, size: (u32, u32)) -> Self {
        Self {
            title,
            size,
        }
    }

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
    
    fn on_start(&self, world: &mut EntityComponentManager) {
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

        glfw.window_hint(glfw::WindowHint::ContextVersion(4, 1));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
        
        #[cfg(target_os = "macos")]
        glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

        let (mut window, events) = glfw
            .create_window(self.size.0, self.size.1, self.title, glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW Window");

        window.make_current();
        window.set_key_polling(true);
        window.set_framebuffer_size_polling(true);

        // Load OpenGL methods...
        gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

        let window_component = Window {
            size:   self.size,
            title:  String::from(self.title),
            event:  events,
            raw:    window,
            glfw:   glfw,
        };

        let e = world.create_entity().unwrap();
        world.add_component(&e, window_component);
    }

    fn on_update(&self, world: &mut EntityComponentManager) {
        match <(Window,)>::query_first_from(&world) {
            Some(entity) => {
                let mut window_component = world.get_component_mut::<Window>(&entity).unwrap();

                if !window_component.raw.should_close() {
                    self.process_events(&mut window_component);
                } else {
                    match <(SystemManagerComponent,)>::query_first_from(&world) {
                        Some(e) => {
                            let mut smc = world.get_component_mut::<SystemManagerComponent>(&e).unwrap();
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