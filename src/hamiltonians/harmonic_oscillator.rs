use super::Hamiltonian;
use crate::particle::Particles;
use crate::wavefunctions::Wavefunction;

pub struct HarmonicOscillator;

impl Hamiltonian for HarmonicOscillator {
    fn compute_kinetic_energy<T>(
        &self,
        wavefunction: &T,
        particles: &Particles,
    ) -> f64
    where
        T: Wavefunction,
    {
        -0.5 * wavefunction.compute_laplacian(particles)
    }

    fn compute_potential_energy<T>(
        &self,
        wavefunction: &T,
        particles: &Particles,
    ) -> f64
    where
        T: Wavefunction,
    {
        2.5
    }
}
