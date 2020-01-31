use crate::hamiltonians::Hamiltonian;
use crate::particle::Particle;
use crate::sampler::Sampler;
use crate::wavefunctions::Wavefunction;

#[derive(Debug)]
pub struct System<T, U> {
    wavefunction: T,
    hamiltonian: U,
    sampler: Sampler,
}

impl<T, U> System<T, U>
where
    T: Wavefunction,
    U: Hamiltonian,
{
    pub fn new(wavefunction: T, hamiltonian: U, sampler: Sampler) -> Self {
        System {
            wavefunction,
            hamiltonian,
            sampler,
        }
    }
}
