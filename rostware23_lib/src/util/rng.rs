use std::thread;
use std::ops::Range;
use std::collections::hash_map::DefaultHasher;
use std::time::SystemTime;
use std::hash::{Hasher, Hash};

// Very simple xorwow RNG implementation because I didn't find a crate
// that did what I wanted
pub struct XorWow {
    state: [u64; 4],
    counter: u64
}

fn get_time_msec() -> u64 {
    let now = SystemTime::now();
    match now.duration_since(SystemTime::UNIX_EPOCH) {
        Ok(t) => t.as_micros() as u64,
        Err(_) => 0,
    }
}

fn gen_seed() -> u64 {
    // generate seed by current time and thread id
    let mut hasher = DefaultHasher::new();
    hasher.write_u64(get_time_msec());
    thread::current().id().hash(&mut hasher);
    hasher.finish()
}

impl XorWow {
     pub fn new() -> Self {
        Self::from_seed(gen_seed())
    }

     pub fn from_seed(seed: u64) -> Self {
        let mut xorwow = Self {
            state: [0, 0, 0, 0],
            counter: 0
        };
        xorwow.seed(seed);
        xorwow
    }

     pub fn seed(&mut self, seed: u64) {
        self.state = [
            0x70A7A712EAF07AA2 ^ seed,
            0xE96A320D4BC6BDDB ^ seed,
            0xBC78C1658C9333BF ^ seed,
            0xBE5B64076E942A9E ^ seed
        ];
        self.counter = 100;
    }

    fn xorwow(&mut self) -> u64 {
        let mut t = self.state[3];
        let s = self.state[0];
        self.state[3] = self.state[2];
        self.state[2] = self.state[1];
        self.state[1] = s;

        t ^= t >> 2;
        t ^= t << 2;
        t ^= s ^ (s << 4);
        self.state[0] = t;

        self.counter = self.counter.wrapping_add(362437);
        t.wrapping_add(self.counter)
    }

     pub fn next(&mut self, range: Range<u64>) -> u64 {
        let num_range = range.end - range.start + 1;
        let random = self.xorwow() % num_range as u64;
        random + range.start
    }
}
