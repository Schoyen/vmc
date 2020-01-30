mod hamiltonians;
mod particle;
mod sampler;
mod system;
mod wavefunctions;

use hamiltonians::HarmonicOscillator;
use particle::Particle;
use sampler::{Sampler, SamplingContainer};
use system::System;
use wavefunctions::Gaussian;
use wavefunctions::Wavefunction;

const HBAR: f64 = 1.0;

fn main() {
    let num_dimensions: usize = 1;
    let num_particles: usize = 10;

    let gaussian = Gaussian::new(1, 2, 3, 2.5);

    let particles = vec![Particle::new(num_dimensions); num_particles];

    let ho = HarmonicOscillator;
    let sampler = SamplingContainer::new();

    let system =
        System::new(gaussian, ho, sampler, num_dimensions, num_particles);
}
