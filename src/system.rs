use crate::hamiltonians::Hamiltonian;
use crate::particle::{Particle, Particles};
use crate::sampler::Sampler;
use crate::wavefunctions::Wavefunction;

#[derive(Debug)]
pub struct System<T, U> {
    wavefunction: T,
    hamiltonian: U,
    particles: Particles,
    sampler: Sampler,
    step_length: f64,
}

impl<T, U> System<T, U>
where
    T: Wavefunction,
    U: Hamiltonian,
{
    pub fn new(
        wavefunction: T,
        hamiltonian: U,
        particles: Particles,
        sampler: Sampler,
        step_length: f64,
    ) -> Self {
        System {
            wavefunction,
            hamiltonian,
            particles,
            sampler,
            step_length,
        }
    }

    pub fn initialize_walkers(&mut self, spread: f64) {
        self.particles.distribute_particles(spread);
    }

    pub fn run_metropolis_steps(&self, num_steps: u64) {
        self.sampler.sample(&self);
    }
}
