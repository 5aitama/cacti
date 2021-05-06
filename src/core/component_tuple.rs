use crate::core::entity::Entity;
use crate::WorldState;

pub trait ComponentTuple {
    fn get_entities(world_state: &WorldState) -> Vec<Entity>;
}

macro_rules! component_tuple {
    ( $( $name:ident )+ ) => {
        impl<$($name: std::any::Any),+> ComponentTuple for ($($name,)+)
        {
            fn get_entities(world_state: &WorldState) -> Vec<Entity> {
                let types = vec![$(std::any::TypeId::of::<$name>()),+];
                world_state.get_entities_with_types(types)
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

