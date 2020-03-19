use super::Hamiltonian;
use crate::particle::Particles;
use crate::wavefunctions::Wavefunction;

pub struct EllipticHarmonicOscillator {
    lambda: f64,
    omega: f64,
}

impl EllipticHarmonicOscillator {
    pub fn new(lambda: f64, omega: f64) -> Self {
        EllipticHarmonicOscillator { lambda, omega }
    }
}

impl Hamiltonian for EllipticHarmonicOscillator {
    fn compute_kinetic_energy<T>(
        &self,
        wavefunction: &T,
        particles: &Particles,
    ) -> f64
    where
        T: Wavefunction,
    {
        -wavefunction.compute_laplacian(particles) * self.omega / 2.0
    }

    fn compute_potential_energy<T>(
        &self,
        _wavefunction: &T,
        particles: &Particles,
    ) -> f64
    where
        T: Wavefunction,
    {
        let mut pos_sq_sum = 0.0;

        for i in 0..particles.get_num_particles() {
            let pos = particles.get_particle_pos(i);

            for j in 0..(pos.len() - 1) {
                pos_sq_sum += pos[j].powi(2);
            }

            pos_sq_sum += pos[pos.len() - 1].powi(2) * self.lambda;
        }

        self.omega * pos_sq_sum / 2.0
    }
}
