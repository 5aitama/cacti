use std::any::{Any, TypeId};

use super::{entity::Entity, managers::{
        component_manager::ComponentsManager, 
        entity_manager::EntityManager, 
        system_manager::SystemManager
    }, sys::Sys};

pub struct EntityComponentManager {
    entity_manager: EntityManager,
    component_manager: ComponentsManager,
}

impl EntityComponentManager {

    pub fn new(max_entities: usize, max_components_type: usize, max_components_per_types: usize) -> Self {
        Self {
            entity_manager: EntityManager::new(max_entities),
            component_manager: ComponentsManager::new(max_entities, max_components_type, max_components_per_types),
        }
    }

    /// Create an `Entity`
    pub fn create_entity(&mut self) -> Option<Entity> {
        self.entity_manager.create_entity()
    }

    /// Destroy an `Entity`
    /// # Arguments
    /// * `entity` - The entity to destroy.
    pub fn destroy_entity(&mut self, entity: &Entity) {
        self.component_manager.on_entity_destroyed(&entity);
        self.entity_manager.destroy_entity(&entity);
    }

    /// Add a component to an entity.
    /// # Arguments
    /// * `entity` - The entity on which we want to add the component.
    /// * `component` - The component to be added to entity.
    pub fn add_component<T>(&mut self, entity: &Entity, component: T) where T: Any {
        self.component_manager.add_component(entity, component);
    }

    /// Remove a component from an entity.
    /// # Arguments
    /// * `entity` - The entity on which we want to remove the component.
    pub fn remove_component<T: Any>(&mut self, entity: &Entity) where T: Any {
        self.component_manager.remove_component::<T>(&entity);
    }

    /// Get reference of an entity component.
    /// # Arguments
    /// * `entity` - The entity that has the component we want to retrieve.
    pub fn get_component<T: Any>(&self, entity: &Entity) -> Option<&T> {
        self.component_manager.get_component::<T>(entity)
    }

    /// Get mutable reference of an entity component.
    /// # Arguments
    /// * `entity` - The entity that has the component we want to retrieve.
    pub fn get_component_mut<T: Any>(&mut self, entity: &Entity) -> Option<&mut T> {
        self.component_manager.get_component_mut::<T>(entity)
    }

    pub fn get_entities_with_type(&self, type_id: &TypeId) -> Option<&Vec<Entity>>{
        self.component_manager.get_entities_with_component(&type_id)
    }

    pub fn get_entities_with_types(&self, type_ids: &Vec<TypeId>) -> Vec<Entity> {
        let mut count = 0usize;

        for type_id in type_ids {
            match self.get_entities_with_type(&type_id) {
                Some(entities) => count += entities.len(),
                _ => {}
            }
        }

        let mut entities = Vec::<Entity>::with_capacity(count);
        
        for type_id in type_ids {
            match self.get_entities_with_type(&type_id) {
                Some(e) => entities.extend(e),
                _ => {}
            }
        }

        entities
    }
}

pub struct World {
    pub entity_component_manager: EntityComponentManager,
    pub system_manager: SystemManager,
}

impl World {

    /// Create new `World`
    /// # Arguments
    /// * `max_entities` - The maximum amount of entities in this world.
    /// * `max_components_type` - The maximum amount of components type in this world.
    /// * `max_components_per_types` - The maximum amount of components per components type.
    /// * `max_systems` - The maximum amount of systems in this world.
    pub fn new(max_entities: usize, max_components_type: usize, max_components_per_types: usize, max_systems: usize) -> Self {
        Self {
            entity_component_manager: EntityComponentManager::new(max_entities, max_components_type, max_components_per_types),
            system_manager: SystemManager::new(max_systems)
        }
    }

    pub fn add_system<T: 'static + Sys>(&mut self, system: T) {
        self.system_manager.register(system)
    }

    pub fn start_loop(&mut self) {
    
        self.system_manager.init(&mut self.entity_component_manager);
        
        loop {
            if !self.system_manager.update(&mut self.entity_component_manager) {
                break;
            }
        }
    }

}

pub trait EntitySelector {
    fn query_from(ecm: &EntityComponentManager) -> Vec<Entity>;
    fn query_first_from(ecm: &EntityComponentManager) -> Option<Entity>;
}

macro_rules! entity_selector {
    ( $( $name:ident )+ ) => {
        impl<$($name: std::any::Any),+> EntitySelector for ($($name,)+)
        {
            fn query_from(ecm: &EntityComponentManager) -> Vec<Entity> {
                let types = vec![$(TypeId::of::<$name>()),+];
                ecm.get_entities_with_types(&types)
            }

            fn query_first_from(ecm: &EntityComponentManager) -> Option<Entity> {
                let entities = Self::query_from(&ecm);
                if entities.len() == 0 {
                    None
                } else {
                    Some(Self::query_from(&ecm)[0])
                }
            }
        }
    };
}

entity_selector! { A }
entity_selector! { A B }
entity_selector! { A B C }
entity_selector! { A B C D }
entity_selector! { A B C D E }
entity_selector! { A B C D E F }
entity_selector! { A B C D E F G }
entity_selector! { A B C D E F G H }
entity_selector! { A B C D E F G H I }
entity_selector! { A B C D E F G H I J }
entity_selector! { A B C D E F G H I J K }
entity_selector! { A B C D E F G H I J K L }