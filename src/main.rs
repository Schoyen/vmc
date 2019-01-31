mod hamiltonians;
mod particle;
mod wavefunctions;

use hamiltonians::hamiltonian;
use wavefunctions::gaussian::Gaussian;
use wavefunctions::wavefunction::Wavefunction;

const HBAR: f64 = 1.0;

fn main() {
    let gaussian = Gaussian::new(1, 2, 3, 2.5);
    dbg!(gaussian);

    let position: Vec<f64> = vec![1.0, 2.0, 3.0];
    let num_dimensions: u8 = 3;

    let particle = particle::Particle::new(position, num_dimensions);
    dbg!(particle);

    let ho = hamiltonians::harmonic_oscillator::HarmonicOscillator;
}
