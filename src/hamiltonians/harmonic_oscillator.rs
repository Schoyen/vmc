use super::Hamiltonian;
use crate::particle::Particles;
use crate::wavefunctions::Wavefunction;

pub struct HarmonicOscillator {
    mass: f64,
    omega: f64,
}

impl HarmonicOscillator {
    pub fn new(mass: f64, omega: f64) -> Self {
        HarmonicOscillator { mass, omega }
    }
}

impl Hamiltonian for HarmonicOscillator {
    fn compute_kinetic_energy<T>(
        &self,
        wavefunction: &T,
        particles: &Particles,
    ) -> f64
    where
        T: Wavefunction,
    {
        -wavefunction.compute_laplacian(particles) / (2.0 * self.mass)
    }

    fn compute_potential_energy<T>(
        &self,
        _wavefunction: &T,
        particles: &Particles,
    ) -> f64
    where
        T: Wavefunction,
    {
        0.5 * self.mass
            * self.omega
            * self.omega
            * particles.compute_pos_squared_sum()
    }
}
