use rand::{
    prelude::{random, thread_rng},
    Rng,
};
use rand_distr::StandardNormal;

use super::MonteCarloMethod;
use crate::particle::Particles;
use crate::wavefunctions::Wavefunction;

pub struct ImportanceSampling {
    diffusion_coefficient: f64,
}

impl ImportanceSampling {
    pub fn new(diffusion_coefficient: f64) -> Self {
        ImportanceSampling {
            diffusion_coefficient,
        }
    }
}

impl MonteCarloMethod for ImportanceSampling {
    fn step<T>(
        &self,
        wavefunction: &T,
        particles: &mut Particles,
        step_length: f64,
    ) -> bool
    where
        T: Wavefunction,
    {
        // Store the previous wavefunction for the ratio test
        let old_wfn = wavefunction.evaluate(&particles);

        // Draw a random particle
        let p_index = random::<usize>() % particles.get_num_particles();
        // Store the old position of the random particle
        let old_pos = particles.get_particle_pos_copy(p_index);

        // Compute the drift force for particle p_index
        let drift_force_old =
            wavefunction.compute_drift_force(p_index, particles);

        let mut rng = thread_rng();
        // Propose a new step and move the particle
        for dim in 0..particles.get_num_dimensions() {
            let gaussian: f64 = rng.sample(StandardNormal);
            let step: f64 =
                self.diffusion_coefficient * drift_force_old[dim] * step_length
                    + gaussian * step_length.sqrt();

            particles.move_particle(step, p_index, dim);
        }

        let new_pos = particles.get_particle_pos(p_index);

        // Evaluate new wavefunction
        let new_wfn = wavefunction.evaluate(&particles);

        // Compute the new drift force for particle p_index
        let drift_force_new =
            wavefunction.compute_drift_force(p_index, particles);

        // Initialize vector accumulators
        let mut first_vector = 0.0;
        let mut second_vector = 0.0;

        /* Compute the exponential terms in the Green's functions */
        for dim in 0..particles.get_num_dimensions() {
            first_vector += (old_pos[dim]
                - new_pos[dim]
                - self.diffusion_coefficient
                    * step_length
                    * drift_force_new[dim])
                .powi(2);
            second_vector += (new_pos[dim]
                - old_pos[dim]
                - self.diffusion_coefficient
                    * step_length
                    * drift_force_old[dim])
                .powi(2);
        }

        // Compute the Green's functions
        let greens_numerator = (-first_vector
            / (4.0 * self.diffusion_coefficient * step_length))
            .exp();
        let greens_denominator = (-second_vector
            / (4.0 * self.diffusion_coefficient * step_length))
            .exp();

        // Compute the ratio test for the Metropolis-Hastings algorithm
        let weight = (greens_numerator / greens_denominator)
            * (new_wfn).powi(2)
            / (old_wfn).powi(2);

        // Perform the Metropolis test
        if weight >= random::<f64>() {
            return true;
        }

        // Reset position as we did not accept the state
        particles.set_particle_pos(p_index, old_pos);

        false
    }
}
