use rand::prelude::random;

use crate::hamiltonians::Hamiltonian;
use crate::particle::Particles;
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

    pub fn compute_local_energy(&self) -> f64 {
        self.hamiltonian
            .compute_local_energy(&self.wavefunction, &self.particles)
    }

    pub fn run_metropolis_steps(
        &mut self,
        num_steps: usize,
        all_dims: bool,
    ) -> Sampler {
        let mut sampler = Sampler::new(num_steps);

        sampler.sample(&self, 0, true);

        for step in 1..num_steps {
            let accepted_step = self.run_metropolis_step(all_dims);

            sampler.sample(&self, step, accepted_step);
        }

        sampler.compute_expectation_values();

        sampler
    }

    fn run_metropolis_step(&mut self, all_dims: bool) -> bool {
        let old_wfn = self.wavefunction.evaluate(&self.particles);

        let p_index = random::<usize>() % self.particles.get_num_particles();
        let pos_copy = self.particles.get_particle_pos(p_index);

        self.particles
            .propose_move(p_index, self.step_length, all_dims);

        let new_wfn = self.wavefunction.evaluate(&self.particles);
        let ratio = new_wfn * new_wfn / (old_wfn * old_wfn);

        if ratio >= random::<f64>() {
            return true;
        }

        self.particles.set_particle_pos(p_index, pos_copy);

        false
    }
}
