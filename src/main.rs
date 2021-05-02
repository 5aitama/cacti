
pub mod core;
use crate::core::{
    world_state::WorldState, 
    component_tuple::ComponentTuple
};

struct Vec2(f32, f32);

struct Cube {
    position: Vec2,
}

struct Sphere {
    position: Vec2,
}

fn move_cube_up(cube: &mut Cube) {
    cube.position.1 += 1.0;
}

fn main() {

    let mut world_state = WorldState::new(64, 64, 64);
    let entity = world_state.create_entity().unwrap();

    world_state.add_component(&entity, Cube { position: Vec2(0., 0.) });
    world_state.add_component(&entity, Sphere { position: Vec2(0., 0.) });

    move_cube_up(world_state.get_component_mut::<Cube>(&entity).unwrap());

    // The components that we want to retrieve
    for component_array in world_state.get_components(&<(Cube, Sphere)>::to_vec()).iter() {
        let components = component_array.get_components_ref();
        for component in components.iter() {
            match component.downcast_ref::<Cube>() {
                Some(r) => println!("Cube position: x:{} y:{}", r.position.0, r.position.1),
                None => match component.downcast_ref::<Sphere>() {
                    Some(r) => println!("Sphere position: x:{}, y:{}", r.position.0, r.position.1),
                    _ => (),
                }
            }
        }
    }
}
