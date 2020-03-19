// Include the content of hamiltonian.rs and harmonic_oscillator.rs
mod elliptic_harmonic_oscillator;
mod hamiltonian;
mod harmonic_oscillator;

// Publicly export the traits and structs
pub use elliptic_harmonic_oscillator::EllipticHarmonicOscillator;
pub use hamiltonian::Hamiltonian;
pub use harmonic_oscillator::HarmonicOscillator;
