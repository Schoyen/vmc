use crate::wavefunctions::wavefunction::Wavefunction;

pub trait Hamiltonian {
    fn compute_kinetic_energy<T>(&self, wavefunction: &T) -> f64
    where
        T: Wavefunction;

    fn compute_potential_energy<T>(&self, wavefunction: &T) -> f64
    where
        T: Wavefunction;

    fn compute_local_energy<T>(&self, wavefunction: &T) -> f64
    where
        T: Wavefunction,
    {
        let kinetic_energy = self.compute_kinetic_energy(wavefunction);
        let potential_energy = self.compute_potential_energy(wavefunction);

        kinetic_energy + potential_energy
    }
}
