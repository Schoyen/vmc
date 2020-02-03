use crate::hamiltonians::Hamiltonian;
use crate::system::System;
use crate::wavefunctions::Wavefunction;

#[derive(Debug)]
pub struct Sampler {
    foo: u32,
}

impl Sampler {
    pub fn new() -> Self {
        Sampler { foo: 0 }
    }

    pub fn sample<T, U>(&self, system: &System<T, U>)
    where
        T: Wavefunction,
        U: Hamiltonian,
    {
    }
}
