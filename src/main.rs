mod hamiltonians;
mod particle;
mod sampler;
mod solvers;
mod system;
mod wavefunctions;

use hamiltonians::{EllipticHarmonicOscillator, HarmonicOscillator};
use particle::Particles;
use solvers::{ImportanceSampling, MetropolisAlgorithm};
use system::System;
use wavefunctions::{Gaussian, InteractingEllipticGaussian};

fn main() {
    let mass = 1.0;
    let omega = 1.0;
    let step_length = 1.0;
    let spread = 2.0;
    let num_dimensions: usize = 3;
    let num_particles: usize = 10;
    let num_metropolis_steps: usize = 1_000_00;

    // Run initial system
    println!(
        "Starting alpha loop for non-interacting system using Metropolis..."
    );
    for alpha in vec![0.2, 0.3, 0.4, 0.5, 0.6, 0.7] {
        let particles = Particles::new(num_particles, num_dimensions);
        let gaussian = Gaussian::new(alpha);

        let ho = HarmonicOscillator::new(mass, omega);

        let mut system = System::new(gaussian, ho, particles, step_length);
        system.initialize_walkers(spread);

        let sampler = system
            .run_metropolis_steps(&MetropolisAlgorithm, num_metropolis_steps);
        sampler.output_statistics(&system);
    }

    // Run initial system with importance sampling
    let imp_step_length = 0.1;
    let diffusion_coefficient = 0.5;

    println!(
        "Starting alpha loop for non-interacting system using Importance..."
    );
    for alpha in vec![0.2, 0.3, 0.4, 0.5, 0.6, 0.7] {
        let particles = Particles::new(num_particles, num_dimensions);
        let gaussian = Gaussian::new(alpha);

        let ho = HarmonicOscillator::new(mass, omega);

        let mut system = System::new(gaussian, ho, particles, imp_step_length);
        system.initialize_walkers(spread);

        let solver = ImportanceSampling::new(diffusion_coefficient);

        let sampler =
            system.run_metropolis_steps(&solver, num_metropolis_steps);
        sampler.output_statistics(&system);
    }

    let num_particles: usize = 10;
    let num_dimensions: usize = 3;
    let a = 0.0043;
    let omega = 1.0;
    let lambda = (8.0 as f64).sqrt();
    let beta = lambda;

    let imp_step_length = 0.1;
    let diffusion_coefficient = 0.5;

    println!("Starting alpha loop for interacting system...");
    for alpha in vec![0.2, 0.3, 0.4, 0.5, 0.6, 0.7] {
        // Run interacting system with importance sampling
        let particles = Particles::new(num_particles, num_dimensions);
        let interacting_elliptic_gaussian =
            InteractingEllipticGaussian::new(alpha, beta, a);

        let eho = EllipticHarmonicOscillator::new(lambda, omega);

        let mut system = System::new(
            interacting_elliptic_gaussian,
            eho,
            particles,
            step_length,
        );
        system.initialize_walkers(spread);

        let sampler = system
            .run_metropolis_steps(&MetropolisAlgorithm, num_metropolis_steps);
        sampler.output_statistics(&system);
        //let solver = ImportanceSampling::new(diffusion_coefficient);

        //let sampler =
        //    system.run_metropolis_steps(&solver, num_metropolis_steps);
        //sampler.output_statistics(&system);
    }
}
