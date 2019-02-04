pub trait Sampler {
    fn new() -> Self;
}

pub struct SamplingContainer {
    foo: u32,
}

impl Sampler for SamplingContainer {
    fn new() -> Self {
        SamplingContainer { foo: 2 }
    }
}
