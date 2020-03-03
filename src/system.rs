use crate::hamiltonians::Hamiltonian;
use crate::particle::Particles;
use crate::sampler::Sampler;
use crate::solvers::MonteCarloMethod;
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

    pub fn compute_local_energy(&self) -> f64 {
        self.hamiltonian
            .compute_local_energy(&self.wavefunction, &self.particles)
    }

    pub fn run_metropolis_steps<S>(
        &mut self,
        solver: &S,
        num_steps: usize,
    ) -> Sampler
    where
        S: MonteCarloMethod,
    {
        let mut sampler = Sampler::new(num_steps);

        sampler.sample(&self, 0, true);

        for step in 1..num_steps {
            let accepted_step = solver.step(
                &self.wavefunction,
                &mut self.particles,
                self.step_length,
            );

            sampler.sample(&self, step, accepted_step);
        }

        sampler.compute_expectation_values();

        sampler
    }
}
