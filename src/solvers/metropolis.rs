use rand::prelude::random;

use super::MonteCarloMethod;
use crate::particle::Particles;
use crate::wavefunctions::Wavefunction;

pub struct MetropolisAlgorithm;

impl MonteCarloMethod for MetropolisAlgorithm {
    fn step<T>(
        &self,
        wavefunction: &T,
        particles: &mut Particles,
        step_length: f64,
    ) -> bool
    where
        T: Wavefunction,
    {
        let old_wfn = wavefunction.evaluate(&particles);

        let p_index = random::<usize>() % particles.get_num_particles();
        let pos_copy = particles.get_particle_pos_copy(p_index);

        for dim in 0..particles.get_num_dimensions() {
            particles.move_particle(
                step_length * 2.0 * (random::<f64>() - 0.5),
                p_index,
                dim,
            );
        }

        let new_wfn = wavefunction.evaluate(&particles);
        let ratio = new_wfn * new_wfn / (old_wfn * old_wfn);

        if ratio >= random::<f64>() {
            return true;
        }

        particles.set_particle_pos(p_index, pos_copy);

        false
    }
}
