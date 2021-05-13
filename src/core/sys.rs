
use super::world::{EntityComponentManager};

pub trait Sys {
    /// Called once after all system was registred.
    fn on_start(&self, _: &mut EntityComponentManager) {}

    /// Called once per frame.
    fn on_update(&self, _: &mut EntityComponentManager) {}
}