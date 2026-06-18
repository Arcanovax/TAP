mod error;
mod loader;

use crate::structures::game::World;
pub use error::ConfigError;
use std::path::Path;

pub fn load(entry: &Path) -> Result<World, ConfigError> {
    loader::load(entry)
}
