use crate::particle::Particles;
use crate::wavefunctions::Wavefunction;

pub trait MonteCarloMethod {
    fn step<T>(
        &self,
        wavefunction: &T,
        particles: &mut Particles,
        step_length: f64,
    ) -> bool
    where
        T: Wavefunction;
}
