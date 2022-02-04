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

fn energy_spherical_osc_variational(
    alpha: f64,
    omega: f64,
    mass: f64,
    num_particles: usize,
    num_dimensions: usize,
) -> f64 {
    (alpha / (2.0 * mass) + mass * omega.powi(2) / (8.0 * alpha))
        * (num_dimensions as f64)
        * (num_particles as f64)
}

fn main() {
    let mass = 1.0;
    let omega = 1.0;
    let step_length = 0.5;
    let spread = 2.0;
    let num_dimensions: usize = 3;
    let num_particles: usize = 10;
    let num_metropolis_steps: usize = 1_000_00;

    let imp_step_length = 0.1;
    let diffusion_coefficient = 0.5;

    let a = 0.0043;
    let lambda = (8.0 as f64).sqrt();
    let beta = lambda;

    for alpha in vec![0.2, 0.3, 0.4, 0.5, 0.6, 0.7] {
        println!("Exact energy for non-interacting, spherical system");
        println!("Alpha: {}", alpha);
        let energy = energy_spherical_osc_variational(
            alpha,
            omega,
            mass,
            num_particles,
            num_dimensions,
        );
        println!("Energy: {}", energy);
        println!("Energy per particle: {}", energy / (num_particles as f64));

        println!("\nNon-interacting system using the Metropolis algorithm");
        let mut system = System::new(
            Gaussian::new(alpha),
            HarmonicOscillator::new(mass, omega),
            Particles::new(num_particles, num_dimensions),
            step_length,
        );
        system.initialize_walkers(spread);

        let sampler = system
            .run_metropolis_steps(&MetropolisAlgorithm, num_metropolis_steps);
        sampler.output_statistics(&system);

        println!("\nNon-interacting system using Importance sampling");
        let mut system = System::new(
            Gaussian::new(alpha),
            HarmonicOscillator::new(mass, omega),
            Particles::new(num_particles, num_dimensions),
            imp_step_length,
        );
        system.initialize_walkers(spread);

        let sampler = system.run_metropolis_steps(
            &ImportanceSampling::new(diffusion_coefficient),
            num_metropolis_steps,
        );
        sampler.output_statistics(&system);

        println!("\nInteracting system using the Metropolis algorithm");
        let mut system = System::new(
            InteractingEllipticGaussian::new(alpha, beta, a),
            EllipticHarmonicOscillator::new(lambda, omega),
            Particles::new(num_particles, num_dimensions),
            step_length,
        );
        system.initialize_walkers(spread);

        let sampler = system
            .run_metropolis_steps(&MetropolisAlgorithm, num_metropolis_steps);
        sampler.output_statistics(&system);

        println!(
            "=================================================================="
        );
    }
}
