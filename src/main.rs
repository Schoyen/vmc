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

fn main() {
    let alpha = 0.4;
    let step_length = 1.0;
    let spread = 2.0;
    let num_parameters: usize = 1;
    let num_dimensions: usize = 1;
    let num_particles: usize = 10;
    let num_metropolis_steps: usize = 10000;

    let particles = Particles::new(num_particles, num_dimensions);
    let gaussian = Gaussian::new(alpha);

    let ho = HarmonicOscillator;

    let mut system = System::new(gaussian, ho, particles, step_length);
    system.initialize_walkers(spread);

    let sampler = system.run_metropolis_steps(num_metropolis_steps);
}
