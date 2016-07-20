pub mod twodim;
pub mod nhood;


use traits::EvolutionState;

pub struct EmptyState;

impl EvolutionState for EmptyState {
    fn update(&mut self) {}
}
