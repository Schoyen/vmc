use crate::hamiltonians::Hamiltonian;
use crate::system::System;
use crate::wavefunctions::Wavefunction;

#[derive(Debug)]
pub struct Sampler {
    num_metropolis_steps: usize,
    num_accepted_steps: usize,
    local_energies: Vec<f64>,
    energy: f64,
    energy_squared: f64,
    energy_var: f64,
}

impl Sampler {
    pub fn new(num_metropolis_steps: usize) -> Self {
        Sampler {
            num_metropolis_steps,
            num_accepted_steps: 0,
            local_energies: vec![0.0; num_metropolis_steps],
            energy: 0.0,
            energy_squared: 0.0,
            energy_var: 0.0,
        }
    }

    pub fn sample<T, U>(
        &mut self,
        system: &System<T, U>,
        step: usize,
        accepted: bool,
    ) where
        T: Wavefunction,
        U: Hamiltonian,
    {
        let local_energy = system.compute_local_energy();

        self.num_accepted_steps += if accepted { 1 } else { 0 };
        self.local_energies[step] = local_energy;
        self.energy += local_energy;
        self.energy_squared += local_energy * local_energy;
    }

    pub fn compute_expectation_values(&mut self) {
        let num_steps = self.num_metropolis_steps as f64;

        self.energy /= num_steps;
        self.energy_squared /= num_steps;
        self.energy_var = self.energy_squared - self.energy * self.energy;
        self.energy_var /= num_steps;
    }

    pub fn output_statistics(&self) {
        println!("Energy: {}", self.energy);
        println!("Energy squared: {}", self.energy_squared);
        println!("Energy variance: {}", self.energy_var);
        println!("Energy standard deviation: {}", self.energy_var.sqrt());
    }
}
