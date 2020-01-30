// Include the content of hamiltonian.rs and harmonic_oscillator.rs
mod hamiltonian;
mod harmonic_oscillator;

// Publicly export the traits and structs
pub use hamiltonian::Hamiltonian;
pub use harmonic_oscillator::HarmonicOscillator;
