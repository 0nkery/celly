pub mod twodim;
pub mod nhood;


use traits::EvolutionState;

/// Dummy evolution state to be used with
/// cellular automata where this concept is
/// not applicable.
pub struct EmptyState;

impl EvolutionState for EmptyState {
    fn update(&mut self) {}
}
