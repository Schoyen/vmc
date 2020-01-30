use crate::hamiltonians::Hamiltonian;
use crate::particle::Particle;
use crate::sampler::Sampler;
use crate::wavefunctions::Wavefunction;

#[derive(Debug)]
pub struct System<T, U, V> {
    wavefunction: T,
    hamiltonian: U,
    sampler: V,
    particles: Vec<Particle>,
}

impl<T, U, V> System<T, U, V>
where
    T: Wavefunction,
    U: Hamiltonian,
    V: Sampler,
{
    pub fn new(
        wavefunction: T,
        hamiltonian: U,
        sampler: V,
        num_dimensions: usize,
        num_particles: usize,
    ) -> Self {
        System {
            wavefunction,
            hamiltonian,
            sampler,
            particles: vec![Particle::new(num_dimensions); num_particles],
        }
    }
}
