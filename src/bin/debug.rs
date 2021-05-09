use cacti::core::{
    world_state::WorldState, 
    system_manager::SystemManager,
};

use cacti::systems::{
    window_sys::WindowSys,
};

use cacti::components::{
    position2d::Position2D,
    tags::cube::TagCube,
    color::Color,
};

extern crate glfw;


fn main() {

    let mut world_state = WorldState::new(64, 64, 64);
    let mut sys_manager = SystemManager::new(16);
    
    // Create an entity and add some components to it !
    let entity = world_state.create_entity().unwrap();

    world_state.add_component(&entity, Position2D(0.0, 0.0));
    world_state.add_component(&entity, Color(1.0, 0.0, 0.0, 0.0));
    world_state.add_component(&entity, TagCube);

    sys_manager.register(WindowSys);

    sys_manager.init(&mut world_state);

    loop {
        if !sys_manager.update(&mut world_state) {
            break;
        }
    }
}