use crate::hamiltonians::Hamiltonian;
use crate::system::System;
use crate::wavefunctions::Wavefunction;

#[derive(Debug)]
pub struct Sampler {
    num_metropolis_steps: usize,
    foo: u32,
    local_energies: Vec<f64>,
}

impl Sampler {
    pub fn new(num_metropolis_steps: usize) -> Self {
        Sampler {
            num_metropolis_steps,
            foo: 0,
            local_energies: vec![0.0; num_metropolis_steps],
        }
    }

    pub fn sample<T, U>(&self, system: &System<T, U>, accepted: bool)
    where
        T: Wavefunction,
        U: Hamiltonian,
    {
    }
}
