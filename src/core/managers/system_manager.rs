
use crate::{components::smc::SystemManagerComponent, core::world::{EntityComponentManager}};
use crate::core::{
    sys::Sys,
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
    /// * `world` - The world state.
    pub fn init(&mut self, world: &mut EntityComponentManager) {
        self.smc_entity = world.create_entity().unwrap();
        world.add_component(&self.smc_entity, SystemManagerComponent { shutdown: false });

        for sys in self.systems.iter() { 
            sys.on_start(world) 
        }
    }

    /// Update all system.
    /// * `world` - The world state.
    pub fn update(&mut self, world: &mut EntityComponentManager) -> bool {
        if world.get_component::<SystemManagerComponent>(&self.smc_entity).unwrap().shutdown {
            return false
        }

        for sys in self.systems.iter() { 
            sys.on_update(world) 
        }

        true
    }
}