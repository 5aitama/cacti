
use crate::components::smc::SystemManagerComponent;
use crate::core::{
    sys::Sys,
    world_state::WorldState,
    entity::Entity,
};

pub struct SystemManager {
    systems: Vec<Box<dyn Sys>>,
    smc_entity: Entity,
}

impl SystemManager {

    /// Create new `SystemManager`.
    /// * `max_systems` - The maximum system amount.
    pub fn new(max_systems: usize) -> SystemManager {
        SystemManager { 
            systems: Vec::with_capacity(max_systems),
            smc_entity: 0,
        }
    }

    /// Register a system.
    /// * `sys` - The system to be register.
    pub fn register<T: 'static + Sys>(&mut self, sys: T) {
        self.systems.push(Box::new(sys));
    }

    /// Init all system.
    /// * `world_state` - The world state.
    pub fn init(&mut self, world_state: &mut WorldState) {
        self.smc_entity = world_state.create_entity().unwrap();
        world_state.add_component(&self.smc_entity, SystemManagerComponent { shutdown: false });

        for sys in self.systems.iter() { 
            sys.on_start(world_state) 
        }
    }

    /// Update all system.
    /// * `world_state` - The world state.
    pub fn update(&mut self, world_state: &mut WorldState) -> bool {
        if world_state.get_component_ref::<SystemManagerComponent>(&self.smc_entity).unwrap().shutdown {
            return false
        }

        for sys in self.systems.iter() { 
            sys.on_update(world_state) 
        }

        true
    }
}