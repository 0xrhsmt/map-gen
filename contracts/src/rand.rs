pub struct MersenneTwister {
    state: [u32; 624],
    index: usize,
}

impl MersenneTwister {
    pub fn new(seed: u32) -> MersenneTwister {
        let mut state = [0; 624];
        state[0] = seed;
        for i in 1..624 {
            let prev = state[i - 1];
            let xored = prev ^ (prev >> 30);
            state[i] = ((xored as u64 * 0x6c078965u64) as u32 + i as u32) & 0xffffffff;
        }
        MersenneTwister { state, index: 0 }
    }

    pub fn generate(&mut self) -> u32 {
        if self.index == 0 {
            self.twist();
        }

        let mut value = self.state[self.index];
        value ^= value >> 11;
        value ^= (value << 7) & 0x9d2c5680;
        value ^= (value << 15) & 0xefc60000;
        value ^= value >> 18;

        self.index = (self.index + 1) % 624;
        value
    }

    pub fn generate_range(&mut self, min: u32, max: u32) -> u32 {
        min + (self.generate() % (max - min + 1))
    }

    fn twist(&mut self) {
        for i in 0..624 {
            let x = (self.state[i] as u64 & 0x80000000u64)
                + (self.state[(i + 1) % 624] as u64 & 0x7fffffffu64);
            let x_a = x >> 1;
            if x % 2 != 0 {
                self.state[i] = (x_a ^ 0x9908b0dfu64) as u32;
            } else {
                self.state[i] = x_a as u32;
            }
        }
    }
}
