use crate::hamiltonians::hamiltonian::Hamiltonian;
use crate::particle::Particle;
use crate::sampler::Sampler;
use crate::wavefunctions::wavefunction::Wavefunction;

pub trait System<T, U, V>
where
    T: Wavefunction,
    U: Hamiltonian,
    V: Sampler,
{
    fn new(
        wavefunction: T,
        hamiltonian: U,
        sampler: V,
        num_particles: u32,
    ) -> Self;
}

pub struct SystemContainer<T, U, V> {
    wavefunction: T,
    hamiltonian: U,
    sampler: V,
    particles: Vec<Particle>,
}

impl<T, U, V> System<T, U, V> for SystemContainer<T, U, V>
where
    T: Wavefunction,
    U: Hamiltonian,
    V: Sampler,
{
    fn new(
        wavefunction: T,
        hamiltonian: U,
        sampler: V,
        num_particles: u32,
    ) -> Self {
        SystemContainer {
            wavefunction,
            hamiltonian,
            sampler,
            particles: Vec::<Particle>::with_capacity(num_particles as usize),
        }
    }
}
