mod hamiltonians;
mod particle;
mod sampler;
mod solvers;
mod system;
mod wavefunctions;

use hamiltonians::HarmonicOscillator;
use particle::Particles;
use solvers::{ImportanceSampling, MetropolisAlgorithm};
use system::System;
use wavefunctions::Gaussian;

fn main() {
    let alpha = 1.1;
    let mass = 1.0;
    let omega = 2.0;
    let step_length = 1.0;
    let spread = 2.0;
    let num_dimensions: usize = 1;
    let num_particles: usize = 10;
    let num_metropolis_steps: usize = 1_000_00;

    let imp_step_length = 0.1;
    let diffusion_coefficient = 0.5;

    let particles = Particles::new(num_particles, num_dimensions);
    let gaussian = Gaussian::new(alpha);

    let ho = HarmonicOscillator::new(mass, omega);

    let mut system = System::new(gaussian, ho, particles, step_length);
    system.initialize_walkers(spread);

    let sampler =
        system.run_metropolis_steps(&MetropolisAlgorithm, num_metropolis_steps);
    sampler.output_statistics();

    let particles = Particles::new(num_particles, num_dimensions);
    let gaussian = Gaussian::new(alpha);

    let ho = HarmonicOscillator::new(mass, omega);

    let mut system = System::new(gaussian, ho, particles, imp_step_length);
    system.initialize_walkers(spread);

    let solver = ImportanceSampling::new(diffusion_coefficient);

    let sampler = system.run_metropolis_steps(&solver, num_metropolis_steps);
    sampler.output_statistics();
}
