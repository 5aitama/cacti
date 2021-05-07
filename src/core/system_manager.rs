
use crate::WorldState;
use crate::Sys;

pub struct SystemManager {
    systems: Vec<Box<dyn Sys>>
}

impl SystemManager {

    /// Create new `SystemManager`.
    /// * `max_systems` - The maximum system amount.
    pub fn new(max_systems: usize) -> SystemManager {
        SystemManager { systems: Vec::with_capacity(max_systems) }
    }

    /// Register a system.
    /// * `sys` - The system to be register.
    pub fn register<T: 'static + Sys>(&mut self, sys: T) {
        self.systems.push(Box::new(sys));
    }

    /// Init all system.
    /// * `world_state` - The world state.
    pub fn init(&mut self, world_state: &mut WorldState) {
        for sys in self.systems.iter() { sys.on_start(world_state) }
    }

    /// Update all system.
    /// * `world_state` - The world state.
    pub fn update(&mut self, world_state: &mut WorldState) {
        for sys in self.systems.iter() { sys.on_update(world_state) }
    }
}