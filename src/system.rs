use rand::{prelude::thread_rng, Rng};

use crate::hamiltonians::Hamiltonian;
use crate::particle::{Particle, Particles};
use crate::sampler::Sampler;
use crate::wavefunctions::Wavefunction;

#[derive(Debug)]
pub struct System<T, U> {
    wavefunction: T,
    hamiltonian: U,
    particles: Particles,
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
        step_length: f64,
    ) -> Self {
        System {
            wavefunction,
            hamiltonian,
            particles,
            step_length,
        }
    }

    pub fn initialize_walkers(&mut self, spread: f64) {
        self.particles.distribute_particles(spread);
    }

    pub fn run_metropolis_steps(&self, num_steps: usize) -> Sampler {
        let sampler = Sampler::new(num_steps);

        sampler.sample(&self, true);

        for step in 0..num_steps {
            sampler.sample(&self, self.run_metropolis_step());
        }

        sampler
    }

    fn run_metropolis_step(&self) -> bool {
        let old_wfn = self.wavefunction.evaluate(&self.particles);
        true
    }
}
