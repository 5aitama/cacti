
use crate::core::{
    component_array::ComponentArray, 
    entity::Entity
};
use bitset::BitSet;
use std::{any::{Any, TypeId}, boxed::Box, collections::HashMap};

pub struct ComponentsManager {
    /// An array that store different type of components array.
    components_arrays: Vec<ComponentArray<Box<dyn Any>>>,

    // Hashmap that contains the id of each type of components.
    components_arrays_id: HashMap<TypeId, usize>,

    /// An array that store the entities signature.
    signatures: Vec<BitSet>,

    /// The maximum amount of components per types.
    max_components_per_types: usize,
}

impl ComponentsManager {
    /// Create new `ComponentsManager`
    /// # Arguments
    /// * `max_entities` - The maximum amount of entities.
    /// * `max_components_type` - The maximum amount of components type.
    /// * `max_components_per_types` - The maximum amount of components per types.
    pub fn new(max_entities: usize, max_components_type: usize, max_components_per_types: usize) -> Self {

        // Init the components arrays
        let mut components_arrays: Vec<ComponentArray<Box<dyn Any>>> = Vec::with_capacity(max_components_type);
        for _ in 0..max_components_type {
            components_arrays.push(ComponentArray::new(max_components_per_types));
        }

        // Init the entities signature
        let mut signatures = Vec::<BitSet>::with_capacity(max_entities);
        for _ in 0..max_entities {
            signatures.push(BitSet::with_capacity(max_components_type));
        }

        Self {
            components_arrays,
            signatures,
            components_arrays_id: HashMap::with_capacity(max_components_type),
            max_components_per_types,
        }
    }

    /// Add a component to an entity.
    /// 
    /// # Arguments
    /// * `entity` - The entity on which we want to add the component.
    /// * `component` - The component to be added to entity.
    pub fn add_component<T: Any>(&mut self, entity: &Entity, component: T) {
        let type_id = TypeId::of::<T>();

        match self.components_arrays_id.get(&type_id) {
            Some(id) => {
                self.signatures[*entity].set(*id, true);
                self.components_arrays[*id].add_component(&entity, Box::new(component));
            },

            None => {
                let id = self.components_arrays.len();
                self.components_arrays.push(ComponentArray::new(self.max_components_per_types));
                self.components_arrays_id.insert(type_id, id);

                self.signatures[*entity].set(id, true);
                self.components_arrays[id].add_component(&entity, Box::new(component));
            },
        };
    }

    /// Remove a component of an entity.
    /// 
    /// # Arguments
    /// * `entity` - The entity on which we want to remove the component.
    pub fn remove_component<T: Any>(&mut self, entity: &Entity) {
        let type_id = TypeId::of::<T>();
        
        match self.components_arrays_id.get(&type_id) {
            Some(id) => {
                self.signatures[*entity].set(*id, false);
                self.components_arrays[*id].rem_component(&entity);
            },
            None => ()
        };
    }

    /// Get immutable reference of an entity component.
    /// 
    /// # Arguments
    /// * `entity` - The entity that has the component we want to retrieve.
    pub fn get_component<T: Any>(&self, entity: &Entity) -> Option<&T> {
        let key = TypeId::of::<T>();
        Some(self.components_arrays[*self.components_arrays_id.get(&key)?].get_component(entity)?.downcast_ref::<T>()?)
    }

    /// Get mutable reference of an entity component.
    /// 
    /// # Arguments
    /// * `entity` - The entity that has the component we want to retrieve.
    pub fn get_component_mut<T: Any>(&mut self, entity: &Entity) -> Option<&mut T> {
        let key = TypeId::of::<T>();
        Some(self.components_arrays[*self.components_arrays_id.get(&key)?].get_component_mut(entity)?.downcast_mut::<T>()?)
    }

    /// Retrieve a reference to a list of entities that have a certain type id.
    /// # Arguments
    /// * `type_id` - 
    pub fn get_entities_with_component(&self, type_id: &TypeId) -> Option<&Vec<Entity>> {
        if let Some(id) = self.components_arrays_id.get(&type_id) {
            Some(self.components_arrays[*id].get_entities())
        } else { 
            None
        }
    }

    /// This function need to be called when an entity was destroyed from the world.
    /// # Arguments
    /// * `entity` - The `Entity` that would be removed.
    pub fn on_entity_destroyed(&mut self, entity: &Entity) {
        // Remove all component that is attached to this entity...
        for (i, component_arrays) in self.components_arrays.iter_mut().enumerate() {
            if self.signatures[*entity].test(i) {
                component_arrays.rem_component(&entity);
            }
        }

        self.signatures[*entity].reset();
    }
}