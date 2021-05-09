use std::any::TypeId;

use crate::core::{
    entity::Entity,
    world_state::WorldState,
};

pub trait ComponentTuple {
    fn get_entities(world_state: &WorldState) -> Vec<Entity>;
    fn get_single_entity(world_state: &WorldState) -> Option<Entity>;
}

macro_rules! component_tuple {
    ( $( $name:ident )+ ) => {
        impl<$($name: std::any::Any),+> ComponentTuple for ($($name,)+)
        {
            fn get_entities(world_state: &WorldState) -> Vec<Entity> {
                let types = vec![$(TypeId::of::<$name>()),+];
                world_state.get_entities_with_types(types)
            }

            fn get_single_entity(world_state: &WorldState) -> Option<Entity> {
                let entities = Self::get_entities(&world_state);
                if entities.len() == 0 {
                    None
                } else {
                    Some(Self::get_entities(&world_state)[0])
                }
            }
        }
    };
}

component_tuple! { A }
component_tuple! { A B }
component_tuple! { A B C }
component_tuple! { A B C D }
component_tuple! { A B C D E }
component_tuple! { A B C D E F }
component_tuple! { A B C D E F G }
component_tuple! { A B C D E F G H }
component_tuple! { A B C D E F G H I }
component_tuple! { A B C D E F G H I J }
component_tuple! { A B C D E F G H I J K }
component_tuple! { A B C D E F G H I J K L }
