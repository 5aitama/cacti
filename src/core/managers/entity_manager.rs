use crate::core::entity::Entity;

pub struct EntityManager {
    /// List of available entities.
    entities_available: Vec<Entity>,

    /// List of all living entities.
    entities_living: Vec<Entity>,
}

impl EntityManager {
    /// Create new `EntityManager`
    /// # Arguments
    /// * `max_entities` - The maximum amount of entities.
    pub fn new(max_entities: usize) -> Self {
        let mut entities_available = Vec::<Entity>::with_capacity(max_entities);
        
        for e in 0..max_entities {
            entities_available.push(e as Entity);
        }

        Self {
            entities_available,
            entities_living: Vec::with_capacity(max_entities),
        }
    }

    /// Create new `Entity`
    pub fn create_entity(&mut self) -> Option<Entity> {
        let entity = self.entities_available.pop()?;
        self.entities_living.push(entity);
        Some(entity)
    }

    /// Destroy an `Entity`.
    /// 
    /// # Arguments
    /// * `entity` - The entity to destroy.
    pub fn destroy_entity(&mut self, entity: &Entity) {
        self.entities_available.push(*entity);
    }
}