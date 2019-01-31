use wavefunctions::wavefunction::Wavefunction;
mod wavefunctions;

fn main() {
    let gaussian = wavefunctions::gaussian::Gaussian::new(1, 2, 3);
    dbg!(gaussian);
}
