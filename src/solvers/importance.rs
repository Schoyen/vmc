use rand::prelude::random;

use super::MonteCarloMethod;
use crate::particle::Particles;
use crate::wavefunctions::Wavefunction;

pub struct ImportanceSampling {
    diffusion_constant: f64,
}

impl ImportanceSampling {
    fn new(diffusion_constant: f64) -> Self {
        ImportanceSampling { diffusion_constant }
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
        false
    }
}
