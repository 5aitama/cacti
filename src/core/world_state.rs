
use std::any::{TypeId, Any};
use std::collections::HashMap;
use crate::core::entity::Entity;
use crate::core::component_array::ComponentArray;

pub struct WorldState {
    /// List of available entities.
    entities_available: Vec<Entity>,
    /// List of all living entities.
    entities_living: Vec<Entity>,
    /// Entities components in the world.
    component_arrays: HashMap<TypeId, ComponentArray<Box<dyn Any>>>,
    /// The maximum amount of components in `ComponentArray`
    max_components_per_type: u32,
}

impl WorldState {
    /// Create new `WorldState`
    /// 
    /// # Arguments
    /// * `max_entities` - The maximum entities in the world
    /// * `max_components` - The maximum components in the world.
    pub fn new(max_entities: u32, max_components_type: u32, max_components_per_type: u32) -> WorldState {
        let mut entities_available = Vec::<Entity>::with_capacity(max_entities as usize);
        
        for e in 0..max_entities {
            entities_available.push(e as Entity)
        }

        WorldState {
            entities_available: entities_available,
            entities_living: Vec::with_capacity(max_entities as usize),
            component_arrays: HashMap::with_capacity(max_components_type as usize),
            max_components_per_type: max_components_per_type,
        }
    }

    pub fn create_entity(&mut self) -> Option<Entity> {
        let entity = self.entities_available.pop()?;
        self.entities_living.push(entity);
        Some(entity)
    }

    /// Destroy an entity.
    /// 
    /// # Arguments
    /// * `entity` - The entity to destroy.
    pub fn destroy_entity(&mut self, entity: &Entity) {
        // Remove all component that is attached to this entity...
        for component_arrays in self.component_arrays.iter_mut() {
            component_arrays.1.rem_component(&entity);
        }

        self.entities_available.push(*entity);
    }

    /// Add a component to an entity.
    /// 
    /// # Arguments
    /// * `entity` - The entity on which we want to add the component.
    /// * `component` - The component to be added to entity.
    pub fn add_component<T: Any>(&mut self, entity: &Entity, component: T) -> bool {
        let key = TypeId::of::<T>();

        if !self.component_arrays.contains_key(&key) {
            self.component_arrays.insert(key, ComponentArray::new(self.max_components_per_type));
        }

        self.component_arrays.get_mut(&key).unwrap().add_component(entity, Box::new(component))
    }

    /// Remove a component of an entity.
    /// 
    /// # Arguments
    /// * `entity` - The entity on which we want to remove the component.
    pub fn rem_component<T: Any>(&mut self, entity: &Entity) -> bool {
        let key = TypeId::of::<T>();
        self.component_arrays.get_mut(&key).unwrap().rem_component(entity)
    }

    /// Get immutable reference of an entity component.
    /// 
    /// # Arguments
    /// * `entity` - The entity that has the component we want to retrieve.
    pub fn get_component_ref<T: Any>(&self, entity: &Entity) -> Option<&T> {
        let key = TypeId::of::<T>();
        Some(self.component_arrays.get(&key)?.get_component_ref(entity).downcast_ref::<T>().unwrap())
    }

    /// Get mutable reference of an entity component.
    /// 
    /// # Arguments
    /// * `entity` - The entity that has the component we want to retrieve.
    pub fn get_component_mut<T: Any>(&mut self, entity: &Entity) -> Option<&mut T> {
        let key = TypeId::of::<T>();
        Some(self.component_arrays.get_mut(&key)?.get_component_mut(entity).downcast_mut::<T>().unwrap())
    }

    pub fn get_components(&self, components_type: &Vec<TypeId>) -> Vec<&ComponentArray<Box<dyn Any>>> {
        let mut components = Vec::<&ComponentArray<Box<dyn Any>>>::new();

        for t in components_type.iter() {
            match self.component_arrays.get(&t) {
                Some(c) => components.push(&c),
                _ => ()
            }
        }
        
        components
    }

    pub fn get_entities_with<A: Any>(&self) -> Vec<Entity> {
        let mut entities = Vec::<Entity>::with_capacity(self.entities_living.len());

        for entity in self.entities_living.iter() {
            match self.get_component_ref::<A>(&entity) {
                Some(_) => entities.push(*entity),
                None => (),
            }
        }

        entities
    }
}