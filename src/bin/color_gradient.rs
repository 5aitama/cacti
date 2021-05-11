use cacti::core::{
    world_state::WorldState, 
    system_manager::SystemManager,
};

use cacti::systems::{
    window_sys::WindowSys,
    before_render_sys::BeforeRenderSys,
    render_sys::RenderSys,
    after_render_sys::AfterRenderSys,
    procedural_square_sys::ProceduralSquareSys,
};

extern crate glfw;


fn main() {

    let mut world_state = WorldState::new(64, 64, 64);
    let mut sys_manager = SystemManager::new(5);

    sys_manager.register(BeforeRenderSys);
    sys_manager.register(RenderSys);
    sys_manager.register(AfterRenderSys);

    sys_manager.register(WindowSys::new("Color Gradient", (800, 600)));
    sys_manager.register(ProceduralSquareSys);

    sys_manager.init(&mut world_state);

    loop {
        if !sys_manager.update(&mut world_state) {
            break;
        }
    }
}