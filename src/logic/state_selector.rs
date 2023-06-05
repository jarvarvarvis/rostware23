use rostware23_lib::game::state::State;

pub trait StateSelector {
    fn should_be_saved(state: &State) -> bool;
}


pub struct AnyStateSelector;

impl StateSelector for AnyStateSelector {
    fn should_be_saved(_: &State) -> bool {
        true
    }
}


pub struct NoStateSelector;

impl StateSelector for NoStateSelector {
    fn should_be_saved(_: &State) -> bool {
        false
    }
}

