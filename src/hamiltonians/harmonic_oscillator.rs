use crate::hamiltonian::Hamiltonian;
use crate::wavefunctions::wavefunction::Wavefunction;

pub struct HarmonicOscillator;

impl Hamiltonian for HarmonicOscillator {
    fn compute_kinetic_energy<T>(&self, wavefunction: &T) -> f64
    where
        T: Wavefunction,
    {
        let mut kinetic_energy = wavefunction.compute_laplacian();
        kinetic_energy *=
            -crate::HBAR.powi(2) / (2.0 * wavefunction.get_mass());

        kinetic_energy
    }

    fn compute_potential_energy<T>(&self, wavefunction: &T) -> f64
    where
        T: Wavefunction,
    {
        2.5
    }
}
