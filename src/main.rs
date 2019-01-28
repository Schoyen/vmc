use crate::wavefunction::Wavefunction;
mod wavefunction;

fn main() {
    let gaussian = crate::wavefunction::Gaussian::new(1, 2, 3);
    dbg!(gaussian);
}
