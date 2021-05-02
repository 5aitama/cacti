pub trait ComponentTuple {
    fn to_vec() -> Vec<std::any::TypeId>;
}

// impl<A, B> Test for (A, B) where A: std::any::Any, B: std::any::Any {
//     fn to_vec() -> Vec<std::any::TypeId>{
//         vec![std::any::TypeId::of::<A>(), std::any::TypeId::of::<B>()]
//     }
// }

macro_rules! component_tuple {
    ( $( $name:ident )+ ) => {
        impl<$($name: std::any::Any),+> ComponentTuple for ($($name,)+)
        {
            fn to_vec() -> Vec<std::any::TypeId> {
                vec![$(std::any::TypeId::of::<$name>()),+]
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