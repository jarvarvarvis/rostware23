use rostware23_lib::game::state::State;

pub trait TranspositionTable {
    fn create_for_depth(depth: i32) -> Self;

    fn add(&mut self, state: State, rating: i32);
    fn contains(&self, state: &State) -> bool;
    fn get(&self, state: &State) -> anyhow::Result<i32>;
}

