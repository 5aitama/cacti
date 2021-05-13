use crate::core::entity::Entity;
use std::any::{Any};

pub struct ComponentArray<T: Any> {
    /// An array that contains the component
    /// index of each entities. The index of
    /// this array is an `Entity`.
    entity_to_component: Vec<usize>,

    /// An array that contains the entity index
    /// from a component index.
    component_to_entity: Vec<Entity>,

    /// The component of each entities.
    components: Vec<Option<T>>,
}

impl<T: Any> ComponentArray<T> {
    /// Create new `ComponentArray`
    /// 
    /// # Arguments
    /// * `max_components` - The maximum components can be store the `ComponentArray`.
    pub fn new(max_components: usize) -> Self {
        let mut etc = Vec::<usize>::with_capacity(max_components);
        let mut cte = Vec::<usize>::with_capacity(max_components);

        for _ in 0..max_components {
            etc.push(0);
            cte.push(0);
        }

        Self {
            entity_to_component: etc,
            component_to_entity: cte,
            components: Vec::with_capacity(max_components),
        }
    }
    
    /// Add a component to an entity.
    /// 
    /// # Arguments
    /// * `entity` - The entity.
    /// * `component` - The component to add.
    pub fn add_component(&mut self, entity: &Entity, component: T) -> bool {

        if *entity > self.components.capacity() {
            return false
        }

        let component_index = self.components.len();

        self.components.push(Some(component));
        self.entity_to_component[*entity] = component_index;
        self.component_to_entity[component_index] = *entity;

        true
    }

    /// Remove an entity component.
    /// 
    /// # Arguments
    /// * `entity` - The entity that has the component to remove.
    pub fn rem_component(&mut self, entity: &Entity) -> bool {

        if *entity > self.components.capacity() {
            return false
        }

        // The last component index
        let last_component_index = self.components.len() - 1;
        // The current component index (component that we want to remove...)
        let curr_component_index = self.entity_to_component[*entity];
        // The last component entity
        let last_entity = self.component_to_entity[last_component_index];

        // Swap the last and current component position in the array
        self.components.swap(curr_component_index, last_component_index);
        
        // Update the last component index
        self.entity_to_component[last_entity] = curr_component_index;
        self.component_to_entity[curr_component_index] = last_entity;

        // Remove the last component in the array
        self.components.remove(last_component_index);

        true
    }

    /// Get immutable reference to an entity component.
    /// 
    /// # Arguments
    /// * `entity` - The entity that own the component.
    pub fn get_component(&self, entity: &Entity) -> Option<&T> {
        Some(self.components[self.entity_to_component[*entity]].as_ref()?)
    }

    /// Get mutable reference to an entity component.
    /// 
    /// # Arguments
    /// * `entity` - The entity that own the component.
    pub fn get_component_mut(&mut self, entity: &Entity) -> Option<&mut T> {
        Some(self.components[self.entity_to_component[*entity]].as_mut()?)
    }

    pub fn get_entities(&self) -> &Vec<Entity> {
        &self.entity_to_component
    }
}