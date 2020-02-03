use crate::particle::Particles;
use crate::wavefunctions::Wavefunction;

pub trait Hamiltonian {
    fn compute_kinetic_energy<T>(
        &self,
        wavefunction: &T,
        particles: &Particles,
    ) -> f64
    where
        T: Wavefunction;

    fn compute_potential_energy<T>(
        &self,
        wavefunction: &T,
        particles: &Particles,
    ) -> f64
    where
        T: Wavefunction;

    fn compute_local_energy<T>(
        &self,
        wavefunction: &T,
        particles: &Particles,
    ) -> f64
    where
        T: Wavefunction,
    {
        let kinetic_energy =
            self.compute_kinetic_energy(wavefunction, particles);
        let potential_energy =
            self.compute_potential_energy(wavefunction, particles);

        kinetic_energy + potential_energy
    }
}
