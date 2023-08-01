use rand::Rng;

pub trait RandomGenerator {
    fn generate(&self, inclusive_min: u32, exclusive_max: u32) -> u32;
}

pub struct PseudoRandomGenerator {}

impl RandomGenerator for PseudoRandomGenerator {
    fn generate(&self, inclusive_min: u32, exclusive_max: u32) -> u32 {
        rand::thread_rng().gen_range(inclusive_min..exclusive_max)
    }
}

pub struct TestRandomGenerator {
    pub expected_min: u32,
    pub expected_max: u32,
    pub return_value: u32,
}

impl RandomGenerator for TestRandomGenerator {
    fn generate(&self, inclusive_min: u32, exclusive_max: u32) -> u32 {
        if inclusive_min != self.expected_min {
            panic!("unexpected min received");
        } else if exclusive_max != self.expected_max {
            panic!("unexpected max received");
        }
        self.return_value
    }
}
