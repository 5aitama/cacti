
pub mod core;

use crate::core::system_manager::SystemManager;
use crate::core::sys::Sys;
use crate::core::{
    world_state::WorldState, 
    component_tuple::ComponentTuple,
};

// Represent 2D coordinates (x, y)
struct Position2D(f32, f32);

// Represent RGBA color.
struct Color(f32, f32, f32, f32);

struct TagCube;

struct MoveCubeSys;
impl Sys for MoveCubeSys {
    fn on_start(&self, _: &mut WorldState) { }
    
    fn on_update(&self, world_state: &mut WorldState) {
        for entity in <(TagCube, Position2D)>::get_entities(&world_state).iter() {
            let mut position2_d = world_state.get_component_mut::<Position2D>(&entity).unwrap();
            position2_d.0 += 6.0;
        }
    }

}

struct PrintSys;
impl Sys for PrintSys {
    fn on_start(&self, _: &mut core::world_state::WorldState) { }

    fn on_update(&self, world_state: &mut core::world_state::WorldState) {

        // Print all entities that have the `Position2D` & `Color` components !
        for entity in <(Position2D, Color)>::get_entities(&world_state).iter() {
            let position2_d = world_state.get_component_ref::<Position2D>(&entity).unwrap();
            let color = world_state.get_component_ref::<Color>(&entity).unwrap();

            println!("Cube position: x:{} y:{}", position2_d.0, position2_d.1);
            println!("color: (r: {}, g: {}, b: {}, a: {})", color.0, color.1, color.2, color.3);
        }

    }

}

fn main() {
    let mut world_state = WorldState::new(64, 64, 64);

    // Create an entity and add some components to it !
    let entity = world_state.create_entity().unwrap();
    world_state.add_component(&entity, Position2D(0.0, 0.0));
    world_state.add_component(&entity, Color(1.0, 0.0, 0.0, 0.0));
    world_state.add_component(&entity, TagCube);

    let mut sys_manager = SystemManager::new(2);

    sys_manager.register(MoveCubeSys);
    sys_manager.register(PrintSys);

    sys_manager.init(&mut world_state);
    sys_manager.update(&mut world_state);
}