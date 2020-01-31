#[derive(Debug)]
pub struct Sampler {
    foo: u32,
}

impl Sampler {
    pub fn new() -> Self {
        Sampler { foo: 0 }
    }
}
