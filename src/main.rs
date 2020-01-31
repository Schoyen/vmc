mod hamiltonians;
mod particle;
mod sampler;
mod system;
mod wavefunctions;

use hamiltonians::HarmonicOscillator;
use particle::{Particle, Particles};
use sampler::Sampler;
use system::System;
use wavefunctions::Gaussian;
use wavefunctions::Wavefunction;

const HBAR: f64 = 1.0;

fn main() {
    let mass = 1.0;
    let num_parameters: usize = 1;
    let num_dimensions: usize = 1;
    let num_particles: usize = 10;

    let particles = Particles::new(num_particles, num_dimensions);
    let gaussian = Gaussian::new(particles, num_parameters, mass);

    let ho = HarmonicOscillator;

    let system = System::new(gaussian, ho, Sampler::new());
}
