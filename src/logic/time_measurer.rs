use std::time::Instant;

pub enum TimeMeasurer {
    Normal { start: Instant, maximum_time_in_millis: u128 },
    Infinite
}

impl TimeMeasurer {
    pub fn new(maximum_time_in_millis: u128) -> Self {
        Self::Normal { 
            start: Instant::now(),
            maximum_time_in_millis
        }
    }

    pub fn new_infinite() -> Self {
        Self::Infinite
    }

    pub fn elapsed_millis_since_start(&self) -> u128 {
        match self {
            TimeMeasurer::Normal { start, maximum_time_in_millis: _ } => start.elapsed().as_millis(),
            TimeMeasurer::Infinite => u128::max_value(),
        }
    }

    pub fn has_time_left(&self) -> bool {
        match self {
            TimeMeasurer::Normal { start: _, maximum_time_in_millis } => self.elapsed_millis_since_start() < *maximum_time_in_millis,
            TimeMeasurer::Infinite => true,
        }
    }
}
