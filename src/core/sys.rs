
use crate::core::world_state::WorldState;

pub trait Sys {
    /// Called once after all system was registred.
    fn on_start(&self, world_state: &mut WorldState);

    /// Called once per frame.
    fn on_update(&self, world_state: &mut WorldState);
}