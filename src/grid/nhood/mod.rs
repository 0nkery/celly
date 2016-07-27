//! Module contains several implemented neighborhoods.

mod moore;
mod von_neumann;

pub use self::moore::MooreNhood;
pub use self::von_neumann::VonNeumannNhood;
