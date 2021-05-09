
use std::any::{TypeId, Any};
use std::collections::HashMap;
use bitset::BitSet;
use crate::core::entity::Entity;
use crate::core::component_array::ComponentArray;

pub struct WorldState {
    /// List of available entities.
    entities_available: Vec<Entity>,

    /// List of all living entities.
    entities_living: Vec<Entity>,

    /// The entities signature.
    signatures: Vec<BitSet>,

    /// Entities components in the world.
    component_arrays: Vec<ComponentArray<Box<dyn Any>>>,
    
    // Hashmap that contains the id of each type of components.
    component_arrays_typeid_id: HashMap<TypeId, usize>,

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
        let mut signatures = Vec::<BitSet>::with_capacity(max_entities as usize);
        
        for e in 0..max_entities {
            entities_available.push(e as Entity);
            signatures.push(BitSet::with_capacity(max_components_type as usize));
        }

        WorldState {
            entities_available: entities_available,
            entities_living: Vec::with_capacity(max_entities as usize),
            component_arrays: Vec::with_capacity(max_components_type as usize),
            component_arrays_typeid_id: HashMap::with_capacity(max_components_type as usize),
            max_components_per_type: max_components_per_type,
            signatures: signatures,
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
        for (i, component_arrays) in self.component_arrays.iter_mut().enumerate() {
            if self.signatures[*entity].test(i) {
                component_arrays.rem_component(&entity);
            }
        }

        self.signatures[*entity].reset();
        self.entities_available.push(*entity);
    }

    /// Add a component to an entity.
    /// 
    /// # Arguments
    /// * `entity` - The entity on which we want to add the component.
    /// * `component` - The component to be added to entity.
    pub fn add_component<T: Any>(&mut self, entity: &Entity, component: T) {
        let t = TypeId::of::<T>();

        match self.component_arrays_typeid_id.get(&t) {
            Some(id) => {
                self.signatures[*entity].set(*id, true);
                self.component_arrays[*id].add_component(&entity, Box::new(component));
            },

            None => {
                let id = self.component_arrays.len();
                self.component_arrays.push(ComponentArray::new(self.max_components_per_type));
                self.component_arrays_typeid_id.insert(t, id);

                self.signatures[*entity].set(id, true);
                self.component_arrays[id].add_component(&entity, Box::new(component));
            },
        };
    }

    /// Remove a component of an entity.
    /// 
    /// # Arguments
    /// * `entity` - The entity on which we want to remove the component.
    pub fn rem_component<T: Any>(&mut self, entity: &Entity) {
        let t = TypeId::of::<T>();
        
        match self.component_arrays_typeid_id.get(&t) {
            Some(id) => {
                self.signatures[*entity].set(*id, false);
                self.component_arrays[*id].rem_component(&entity);
            },
            None => ()
        };
    }

    /// Get immutable reference of an entity component.
    /// 
    /// # Arguments
    /// * `entity` - The entity that has the component we want to retrieve.
    pub fn get_component_ref<T: Any>(&self, entity: &Entity) -> Option<&T> {
        let key = TypeId::of::<T>();
        Some(self.component_arrays[*self.component_arrays_typeid_id.get(&key)?].get_component_ref(entity)?.downcast_ref::<T>()?)
    }

    /// Get mutable reference of an entity component.
    /// 
    /// # Arguments
    /// * `entity` - The entity that has the component we want to retrieve.
    pub fn get_component_mut<T: Any>(&mut self, entity: &Entity) -> Option<&mut T> {
        let key = TypeId::of::<T>();
        Some(self.component_arrays[*self.component_arrays_typeid_id.get(&key)?].get_component_mut(entity)?.downcast_mut::<T>()?)
    }

    fn get_type_id(&self, t: &TypeId) -> Option<&usize> {
        Some(self.component_arrays_typeid_id.get(&t)?)
    }

    pub fn get_entities_with_types(&self, t: Vec<TypeId>) -> Vec<Entity> {
        let mut entities = Vec::<Entity>::with_capacity(self.entities_living.len());
        for entity in self.entities_living.iter() {
            let mut have_components = true;
            for t in t.iter() {
                if self.get_type_id(&t).is_none() {
                    have_components = false;
                    break;
                }

                if !self.signatures[*entity].test(*self.get_type_id(&t).unwrap()) {
                    have_components = false;
                    break;
                }
            }

            if have_components {
                entities.push(*entity);
            }
        }

        entities
    }

    pub fn has_component(&self, entity: &Entity, component_id: usize) -> bool {
        self.signatures[*entity].test(component_id)
    }
}