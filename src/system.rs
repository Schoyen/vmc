use crate::hamiltonians::hamiltonian::Hamiltonian;
use crate::sampler::Sampler;
use crate::wavefunctions::wavefunction::Wavefunction;

pub trait System<T, U, V>
where
    T: Wavefunction,
    U: Hamiltonian,
    V: Sampler,
{
    fn new(wavefunction: T, hamiltonian: U, sampler: V) -> Self;
}

pub struct SystemContainer<T, U, V> {
    wavefunction: T,
    hamiltonian: U,
    sampler: V,
}

impl<T, U, V> System<T, U, V> for SystemContainer<T, U, V>
where
    T: Wavefunction,
    U: Hamiltonian,
    V: Sampler,
{
    fn new(wavefunction: T, hamiltonian: U, sampler: V) -> Self {
        SystemContainer {
            wavefunction,
            hamiltonian,
            sampler,
        }
    }
}
